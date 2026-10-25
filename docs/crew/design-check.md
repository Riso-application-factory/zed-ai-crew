# Crew design check against the Zed fork

| Field | Content |
| --- | --- |
| Version | 1.1 · 2026-10-25 |
| Question | Do the claims about Zed that the tech spec (v2.2) relies on hold in the code of this fork? |
| Basis | Fork `main` at `11caaae`. ACP crates `agent-client-protocol` 3.4.0 and `agent-client-protocol-schema` 1.12.0 from crates.io. Adapters `@agentclientprotocol/claude-agent-acp@0.95.0` and `@agentclientprotocol/codex-acp@2.2.2` from npm. Nothing was built or run. |
| Method | Four readers worked in parallel (thread lifecycle, ACP client, infrastructure, adapter behaviour under Zed). The author then re-read the load-bearing findings in the code; those are marked **re-checked**. |
| Line numbers | All paths and lines refer to the fork. Spec commit `347f171` is not in this (shallow) clone; the fork's upstream base appears to be about 2026-10-17. Spec lines still match in slow-moving files but have moved in `v1.rs`, `acp_thread.rs`, and `agent_ui` (see the line map at the end). |
| Status | The Human approved all 24 proposals (option A) on 2026-10-25. Tech spec v2.3, plan 1.2, `open-question.md` 1.1 and `spike-list.md` 1.1 apply them; this report keeps the evidence. |

## Summary

The core architecture holds. Every production path that creates, loads, or resumes an external-agent thread goes through `Agent::server()`. The ACP crate already has `system_prompt` and `_meta` on new, load, and resume, so H2 is feasible. Zed never auto-answers an external agent's permission request. And under Zed, the Claude and Codex adapters still run Read, Write, Edit, and Bash in their own processes, so the spike's sandbox results carry over.

Twenty-four findings change the design. Seven are high severity:

| # | Finding | Severity |
| --- | --- | --- |
| D1 | Questions an agent asks the user (elicitations) bypass the gate | High |
| D2 | Zed accepts file writes outside the project through invisible worktrees, with no permission step | High |
| D3 | On the fork's default build channel, credentials are not stored in the Keychain | High |
| D4 | Crew agent ids need more plumbing: store registration, the stored thread id, stock-id special cases, settings overrides | High |
| D5 | Showing a Crew thread makes the Crew id the panel's selected agent | High |
| D6 | Read-only worker threads cannot reuse the subagent view | High |
| D7 | Codex requests that widen the sandbox look like ordinary command approvals | High |

## Findings that change the design

### D1 · Elicitations bypass the gate (high, re-checked)

- Zed advertises form and URL elicitation to every agent (`crates/agent_servers/src/acp/v1.rs:470-474`).
- The Claude adapter therefore keeps its `AskUserQuestion` tool enabled; it disables the tool only when form elicitation is missing (`claude-agent-acp dist/acp-agent.js:8586`). Questions, MCP elicitations, and fallback dialogs arrive as `elicitation/create` (`acp-agent.js:7779-7783, 8710-8724`). Codex sends `request_user_input` the same way, possibly with no timeout (`codex-acp dist/index.js:32938-32961`).
- Zed surfaces these as `AcpThreadEvent::ElicitationRequested` (`crates/acp_thread/src/acp_thread.rs:3581`), not as `ToolAuthorizationRequested`. The gate of spec 10.1 never sees them, a worker shown without a composer cannot be answered, and an answer given in a form is a decision that DecisionCapture does not record.

**Change.** Add `AskUserQuestion` to `disallowedTools` in the `_meta.claudeCode.options` of every Crew session (the adapter merges client `disallowedTools` at `acp-agent.js:8730`). Crew subscribes to `ElicitationRequested` on its sessions and declines it; workers ask through a `needs_context` handoff and the Orchestrator through `crew_ask_human`. Contract tests cover both adapters.

### D2 · File writes outside the project through invisible worktrees (high, re-checked)

- When a tool call names an absolute path outside the project as a location, Zed opens it with `open_local_buffer` (`acp_thread.rs:1759-1760`), which calls `find_or_create_worktree(path, false, …)` and creates an invisible worktree (`crates/project/src/project.rs:3205`).
- `resolve_locations` runs on every tool call and tool-call update, including tool calls inside permission requests (`acp_thread.rs:5419, 5589, 5654`). Rejecting the permission does not undo the worktree.
- `fs/write_text_file` is applied with no permission step (`v1.rs:9416-9449`). Its path check accepts any worktree, invisible ones included (`acp_thread.rs:7342` → `crates/project/src/worktree_store.rs:469-472`), and matches by path prefix without resolving symlinks (`crates/util/src/paths.rs:220-222`).
- Impact: spec 6.1 "Antigravity writes only inside the project's worktree" does not hold; the Antigravity Orchestrator of spec 7.5 cannot be kept read-only by the gate, because a direct write never asks; spec 10.4 understates the risk.

**Change.** Add a hook point (H12) in the ACP client's fs handlers: for sessions Crew owns, consult Crew's policy before every `fs/write_text_file` and `fs/read_text_file` (allow only canonical paths inside the worker's worktree; deny for the Orchestrator, probe sessions, and read-only workers), and stop turning outside locations into worktrees for those sessions. Until H12 exists, record this as a residual risk and keep Antigravity off tasks that need write isolation.

### D3 · Credentials are not in the Keychain on the dev channel (high, re-checked)

- `crates/zed/RELEASE_CHANNEL` is `dev`. On the Dev channel, Zed's credentials provider uses `DevelopmentCredentialsProvider` unless `ZED_DEVELOPMENT_USE_KEYCHAIN` is set (`crates/zed_credentials_provider/src/zed_credentials_provider.rs:46-62`); the readers report that it stores credentials in a plaintext file.
- NEED 05 and spec 12 require the GitHub App private key in the macOS Keychain; H11 stores the Codex API key the same way.

**Change.** Crew stores its secrets through the Keychain provider whatever the release channel, or refuses to store a secret when the active provider is not the Keychain. T4.2 gets an acceptance test for this. Record which release channel the Human's daily fork build uses, because the channel also decides staff behaviour for feature flags.

### D4 · Crew agent ids need more plumbing (high, partly re-checked)

- **Registration.** The connection store keeps only custom ids present in the agent server store's `external_agents` (`crates/agent_ui/src/agent_connection_store.rs:277`), which `reregister_agents` empties and rebuilds from settings and the registry (`crates/project/src/agent_server_store.rs:348`). The first server cached per key wins (`agent_connection_store.rs:149-151`). Crew must register its ids in the store (H10) before `AgentPanel::load`, re-register after every rebuild, and register its factory (H5d) before the panel restores threads.
- **Stored id** (re-checked). `AcpThread` holds the inner `AcpConnection` (`v1.rs:919-923`), and the thread's stored agent id comes from that connection (`crates/agent_ui/src/thread_metadata_store.rs:1305`: `thread_ref.connection().agent_id()`). The inner connection must be created with the Crew id, or reopening routes the thread to the stock agent.
- **Stock-id special cases** (re-checked). Zed clears `ANTHROPIC_API_KEY` only for the registry agent `claude-acp` (`crates/agent_servers/src/custom.rs:236-237`); Codex, Gemini, and Cursor have similar cases (`custom.rs:239-248`, `v1.rs:451, 649`). Without copying them, the Claude adapter behind `crew-claude-acp` may authenticate with an API key from the environment instead of the logged-in plan (NEED 25).
- **Settings overrides.** `AcpConnectionDefaults::observe_settings` applies the `agent_servers.<agent id>` settings asynchronously after new, load, and resume, overriding mode and model without an event (`v1.rs:156-168, 1310-1345`). Crew ids must not pick up such settings, or ModeGuard must re-verify after they apply.

### D5 · Showing a Crew thread changes the selected agent (high, re-checked)

`create_thread_with_options` restores the selected agent when the caller overrides it (`crates/agent_ui/src/agent_panel.rs:3233-3265`), but `set_base_view` sets `selected_agent` to the thread's agent whenever that thread becomes the visible view, and persists it (`agent_panel.rs:4354`). After the Human looks at a Crew thread, the next "+" creates a new Crew session through `CrewConnection::new_session`. Spec 7.1's note is incomplete.

**Change.** Extend H5: Crew agent ids never become the selected agent, and `CrewConnection::new_session` creates a session only when called from "Crew: New Session".

### D6 · Read-only worker threads need new core support (high, re-checked)

- `is_subagent` is `parent_session_id.is_some()` (`crates/agent_ui/src/conversation_view/thread_view.rs:1268-1269`), set only in `AcpThread::new` (`acp_thread.rs:3686`); `AcpConnection` passes `None` and there is no setter. Faking a parent id stops the sidebar from saving the thread (`conversation_view.rs:1850`).
- In a root thread view, the "allow always" keybinding answers the first pending request of any session in that conversation (`conversation_view.rs:463-486`, `thread_view.rs:3012-3017`). With workers shown as children and the gate answering asynchronously, one keystroke could choose `allow_always`.

**Change.** H5b becomes an explicit read-only flag for a thread or its view. The gate answers worker permission requests before they render, and Crew sessions never offer `allow_always` options in the UI.

### D7 · Codex sandbox-widening requests look like command approvals (high)

Requests that carry `additionalPermissions` or a network context arrive as `kind: execute` with an `allow_once` accept option (`codex-acp dist/index.js:28425, 32091, 32150-32156`). Permission-profile requests offer `allow_permissions_turn` as `allow_once` (`index.js:32228-32234`). Spec 10.1 chooses options by `kind`, which is still right for choosing which option to send, but "deny sandbox-widening requests" cannot be decided from `kind`.

**Change.** The gate classifies each request from its `rawInput` (additional permissions, network context, permission profile) before choosing an option. Contract tests capture these request shapes.

### D8 · Worker worktree creation (medium-high, re-checked)

- `create_worktree_workspace` needs a `Workspace` and a `Window` whose root is a `MultiWorkspace` (`crates/git_ui_core/src/worktree_service.rs:736-742, 1164`). It refuses to run while another creation is in progress (`:781`), creates detached worktrees (`:504`), adds non-git folders unchanged, and adds a full Workspace and Project per worker to the same window and its sidebar (`:1365-1367`).
- A window-free path exists: `GitStore::create_worktree` with a `NewBranch` target (`crates/project/src/git_store.rs:9276`), `Project::local` (`project.rs:1232`, already used headless by `crates/eval_cli/src/main.rs:735`), then `new_session(project, …)`. Zed's own `create_thread` tool already spawns threads in background worktree workspaces (`agent_panel.rs:4891-4953`).

**Change.** Spec 8 and 9.5: create each worker worktree through `GitStore` on a named branch, give it a headless `Project`, and show the worker thread read-only from the Orchestrator's workspace. If `create_worktree_workspace` is kept, spawns are serialized.

### D9 · The in-process MCP server needs more than the spec says (medium-high, re-checked)

- `McpServer` answers only `tools/call`, `tools/list`, and registered handlers; anything else gets "unhandled method" (`crates/context_server/src/listener.rs:214-242`). Crew must register an `initialize` handler itself.
- The socket's temp directory is created with tempfile's default mode, 0755 per the reader, not the 0700 of spec 2.3; Crew must restrict it.
- Connections keep their tool map after the server is dropped (`listener.rs:206-242`), so dropping the server does not revoke access. On Stop, Crew kills the adapter process or its tools check the session state.
- Framing is newline-delimited JSON-RPC (`listener.rs:360-394`), so the stdio bridge (H1) can copy bytes.

### D10 · Run store location and durability (medium, re-checked)

`db::static_connection!` uses Zed's shared per-app database (`crates/db/src/db.rs:266-268`), so the run store is not "a separate SQLite" (spec 4.1); a separate file needs `open_db::<M>` (`db.rs:174`). With `ZED_STATELESS`, or when opening fails, the database silently falls back to memory (`db.rs:178-180, 197-202`). Readiness and Recovery must detect a non-durable store and stop.

### D11 · Provider subagent sessions load through the Crew connection (medium)

Sessions an adapter reports in `_meta.subagent_session_info` are loaded on the same connection (`conversation_view.rs:1590-1620, 2438`), and the load error is discarded. Spec 3.3's "reject unknown session ids" would make those views fail silently. Accept child sessions of a known Crew session and record them, or disable provider subagents in Crew sessions.

### D12 · DecisionCapture has no stable message id (medium)

A new user message emits `NewEntry` with `AgentThreadEntry::UserMessage`, whose identity is `MessageIdentity::Legacy(None)` (`acp_thread.rs:6559-6566, 5160`); `SubmissionId` is an in-memory counter (`crates/acp_thread/src/submission.rs:20`). The composer and programmatic prompts both call `thread.send` (`thread_view.rs:2110`), so there is no origin flag. Crew assigns its own message ids, records the submission id of every prompt it sends, and treats the other user entries of the Orchestrator thread as the Human's. This answers the DecisionCapture event question of `open-question.md` section 3.

### D13 · ModeGuard sees only agent-originated changes (medium)

`ModeUpdated` and `ConfigOptionsUpdated` (`acp_thread.rs:3596-3597`) fire only when the agent reports a change (`:4504-4515`). Changes made on Zed's side (the mode selector `v1.rs:9218`, `set_config_option` `:180`, settings defaults `:1310-1345`) emit nothing. ModeGuard also reads `session_modes()` and the config state after each such point.

### D14 · Dropped updates cannot be logged at the thread level (medium)

The catch-all arm exists (`acp_thread.rs:4526`) and top-level usage `_meta` is dropped (`:4522`, `update_usage` `:5366-5379`), so H4(a) is right. But unknown session-update types already fail to parse inside the SDK (`agent-client-protocol-3.4.0 src/jsonrpc/handlers.rs:272-279`; the v1 `SessionUpdate` enum has no fallback variant), so H4(b) would only see known variants. Logging dropped updates (NEED 43) needs the SDK layer or `AcpConnection::subscribe_debug_messages()` (`v1.rs:488-495`), a debug facility whose format may change.

### D15 · Restricted Mode hides project settings (medium)

For an untrusted worktree, Zed holds back `.zed/settings.json` (`crates/project/src/project_settings.rs:1362-1384`), so `local_settings()` is empty. Crew must detect this and stop instead of running without project policy. The new `crew` key also needs adding to `flattened_deserialize!` (`crates/settings_content/src/project.rs:90-96`).

### D16 · Additional directories widen Codex's write scope (medium)

Zed sends every work directory after the first as `additionalDirectories` (`v1.rs:1157-1175`), and Codex adds them to its writable roots (`codex-acp dist/index.js:35878-35885`). Crew worker sessions send none.

### D17 · Claude reads settings files from the worktree and the user (medium)

The adapter loads `settingSources: ["user", "project", "local"]` (`acp-agent.js:8668`). A `.claude/settings*.json` left in a worktree, or the Human's own user settings, can change a worker's permissions. Set `settingSources` explicitly for Crew sessions or reset `.claude/` per worktree, and contract-test that Crew's deny rules win.

### D18 · Verification sandbox needs a scratch directory (low-medium)

Use the public `Sandbox` API (`crates/sandbox/src/sandbox.rs:519, 611, 638`); `wrap_invocation` is in a private module (`sandbox.rs:23-24`). Seatbelt allows writes only to listed paths and `/dev` (`macos_seatbelt.rs:310-341`), so builds need a writable scratch directory set as `TMPDIR`, and possibly unix sockets.

### D19 · Feature-flag rollback needs a rebuild (low)

Once `enabled_for_all()` returns true, settings cannot switch Crew off (`crates/feature_flags/src/store.rs:166-182`). Brief 9's rollback is a rebuild with the flag off.

### D20 · Hook locations H6 and H9 (low)

`crew_ui::init(cx)` belongs in `crates/zed/src/main.rs` next to `agent_ui::init` (`:713`), not in `zed.rs`. Onboarding is a single page with no page registry (`crates/onboarding/src/onboarding.rs:299-301`); H6 edits that page or opens a separate Crew item.

### D21 · Side effects of the wrapper (low)

- The only downcast of a wrapped connection is `agent_connection_store.rs:115`; Crew connections would disappear from the ACP Logs view (`crates/acp_tools/src/acp_tools.rs:133`).
- The trait has 26 methods with default implementations (`crates/acp_thread/src/connection.rs:95-305`); a wrapper that forgets one still compiles. Add a test that checks forwarding.
- Per-thread calls (prompt, cancel, modes, config options, close) go to the inner connection that `AcpThread` holds (re-checked, `v1.rs:919-923`); `CrewConnection` sees only new, load, resume, authentication, session lists, and elicitation routing. Spec 3.3's "the remaining methods forward to `inner`" should describe this.

### D22 · An inactive ACP v2 connection exists upstream (low)

`AcpV2Connection` (`crates/agent_servers/src/acp/v2.rs:119, 969`) is exported but unused; `acp::connect` returns v1 (`v1.rs:320-340`). If upstream switches, H2, H3, and the `Rc<AcpConnection>` inside `CrewConnection` break. The upstream sync (T1.14) watches for it.

### D23 · Zed advertises an unsandboxed terminal (low)

Zed runs `terminal/create` without a sandbox or a permission step (`v1.rs:9650-9709`). The pinned adapters never use it (nor `fs/*`), but an update could. The contract tests send exactly `client_capabilities_for_agent` and assert that no `fs/*` or `terminal/*` request arrives.

### D24 · The spec was written against a newer upstream than the fork (info)

See "Line numbers" above and the line map below. T0.2 should start from this report.

## Claims that hold

| Claim (spec) | Evidence |
| --- | --- |
| `CustomAgentServer::new` is constructed only in `Agent::server()`; every production external connection goes through it (3.3, T9) | `crates/agent_ui/src/agent_ui.rs:586`; the only production `AgentServerDelegate::new` is `agent_connection_store.rs:297` |
| Every reopen path (sidebar, archive, restart restore, reconnect, evicted idle thread) loads or resumes through `Agent::server()` (3.3, H5d) | `conversation_view.rs:1360-1371, 1211-1268`; extra site for subagents at `:2438` |
| Thread import lists sessions from every cwd (H5e) | `crates/agent_ui/src/thread_import.rs:803-805` |
| Test infrastructure with a wrapping fake connection exists (17) | `v1.rs:1708, 1767, 1841-1842` |
| `new_session`, `load_session`, `resume_session` send only cwd, additional directories, and enabled project MCP servers (H2) | `v1.rs:1130-1153`; `mcp_servers_for_project` at `v1.rs:9134` |
| The ACP schema has `system_prompt` and `meta` on new, load, and resume with builders; Zed's `unstable` feature enables them (H2) | schema `src/v1/agent.rs:824-836, 1015-1027, 1401-1413`; crate `Cargo.toml:58-66` |
| The SDK can register extension notifications and send extension requests (H3, spike S6) | crate `src/jsonrpc.rs:1579` (`on_receive_notification`), `:4300` (`send_request`), `:5676-5685` (`UntypedMessage`); H3 needs a core edit because `AcpConnection.connection` is private (`v1.rs:75`) |
| Permission requests keep each option's ACP kind and can be answered programmatically; no setting auto-approves external agents (10.1) | `v1.rs:9284-9300, 9344`; `acp_thread.rs:5837, 6092`; `connection.rs:773`. Zed itself answers only unknown kinds (error) and send/retry/cancel (`cancelled`) |
| Claude and Codex run Read, Write, Edit, and Bash in their own processes under Zed, so the spike's sandbox results carry over (6.1, 6.2) | `acp-agent.js:8591-8592, 8877-8884`; no `fs/*` or `terminal/*` calls in either adapter; `codex-acp dist/index.js:35610` |
| `--askpass` is one-directional and dispatched before the app starts (H1) | `crates/zed/src/main.rs:213-220`; `crates/askpass/src/askpass.rs:419, 433-469` |
| Feature-flag overrides apply only in debug builds or for staff; debug counts as staff (H8) | `crates/feature_flags/src/store.rs:105-107, 166-182` |
| `ProjectSettingsContent` is flattened into `SettingsContent`; unmerged user and local settings APIs exist (H7, 5.3) | `crates/settings_content/src/settings_content.rs:175-176`; `crates/settings/src/settings_store.rs:506, 1110, 1198` |
| npx agents install with a version range; binary agents delete other versions in the same directory; a separate id gets its own directory (H10) | `agent_server_store.rs:1467, 1099-1103, 403-405, 1387-1390` |
| `custom.rs` forwards the wrong name `OPEN_AI_API_KEY` (H11) | `custom.rs:243-244` (small impact: the adapter inherits Zed's environment) |
| The `Sandbox` API limits writes per call and sets network off, on, or to an allowlist (10.3) | `sandbox.rs:85-118` |
| `jsonwebtoken`, `rsa`, `octocrab`, `sha2` are workspace dependencies (12) | `Cargo.toml:680, 806, 860, 885` |

## Open items this report answers

- The DecisionCapture event question (item 3 of `open-question.md` section 3 in version 1.0): D12.
- Whether Antigravity can write through a hidden worktree (item 6 of the same list): yes, see D2.
- Spike S6: the SDK API is identified (see "Claims that hold"); a running test against a fake agent is still needed.
- Spike S7: framing is newline-delimited JSON-RPC and Crew must handle `initialize` (D9); running the three providers against the bridge is still needed.

## Proposed spec changes

| Spec section | Change | From |
| --- | --- | --- |
| 2.3 | Socket directory mode is set to 0700 by Crew | D9 |
| 3.2 | H3 edits `v1.rs` because the connection is private; H4(b) moves to the SDK layer or the debug stream; H5 adds "Crew ids never become the selected agent" and a real read-only flag (H5b); H5d and H10 register Crew ids in the agent server store before `AgentPanel::load` and after each rebuild; H9 is in `main.rs`; new H12 (fs policy for Crew sessions) | D2, D4–D6, D14, D20 |
| 3.3 | The inner connection carries the Crew id; per-thread calls bypass the wrapper; child sessions of a known Crew session are accepted; copy stock-id special cases; ignore `agent_servers.<id>` settings | D4, D11, D21 |
| 4.1 | The run store lives in Zed's shared database or in its own file through `open_db`; check durability | D10 |
| 5.3 | Detect Restricted Mode; add `crew` to `flattened_deserialize!` | D15 |
| 6.1, 6.2 | Antigravity row; `disallowedTools: ["AskUserQuestion"]`, explicit `settingSources`, no `additionalDirectories` in worker payloads | D1, D2, D16, D17 |
| 7.1, 7.2, 7.4, 7.5 | Selected-agent fix; `initialize` handler and revocation on Stop; Crew-assigned message ids; Antigravity Orchestrator caveat until H12 | D2, D5, D9, D12 |
| 8, 9.5 | Worker worktrees through `GitStore` on a named branch with a headless `Project` | D8 |
| 10.1, 10.2, 10.3, 10.4 | Elicitations; classify by `rawInput`; no `allow_always` in Crew UIs; ModeGuard reads state; `Sandbox` API with scratch `TMPDIR`; updated residual risks | D1, D2, D6, D7, D13, D18 |
| 12 | Secrets go to the Keychain whatever the release channel | D3 |
| 14.1 | Recovery stops on a non-durable store | D10 |
| 17 | Contract tests: no `fs/*` or `terminal/*` traffic, elicitation handling, Codex widening requests, deny-rule precedence | D1, D7, D17, D23 |
| B.1 | Replace line numbers with the fork's (line map below) | D24 |

## Line map (spec → fork)

| Item | Spec | Fork |
| --- | --- | --- |
| `AcpConnection::new_session` | `v1.rs:1298` | `v1.rs:1238` |
| `load_session` / `resume_session` | — | `v1.rs:1418` / `1457` |
| `mcp_servers_for_project` | `v1.rs:9651` | `v1.rs:9134` |
| Client builder handlers | `v1.rs:428-476` | `v1.rs:368-442` |
| `handle_request_permission` | `v1.rs:9837` | `v1.rs:9320` |
| `AcpThreadEvent` | `acp_thread.rs:3698` | `acp_thread.rs:3569-3599` |
| `handle_session_update` catch-all | `acp_thread.rs:4902` | `acp_thread.rs:4526` |
| `update_usage` | `acp_thread.rs:5892` | `acp_thread.rs:5366` |
| `authorize_tool_call` | `acp_thread.rs:6605` | `acp_thread.rs:6077` |
| fs read / write handlers | `acp_thread.rs:7822-7843` / `7902-7916` | `acp_thread.rs:7250-7272` / `7330-7345` |
| `AgentConnection::into_any` | `connection.rs:309` | `connection.rs:304` |
| `is_subagent` | `thread_view.rs:1529` | `thread_view.rs:1268` |
| `CustomAgentServer::new` in `Agent::server()` | `agent_ui.rs:587` | `agent_ui.rs:586` |

Unchanged: `custom.rs:240-245`, `agent_server_store.rs:1055/1309/1456`, `worktree_service.rs:736`, `db.rs:247`, `store.rs:105-107/165-190`, `listener.rs:34/402-423`, `askpass.rs:278-281/414-469`, `main.rs:215-220`, `worktree_store.rs:482-488`.

## Not verified

- How the Antigravity adapter routes reads, writes, and commands (its code was not available); the "Baseline" rows of spec 6.1 stand as they are.
- Whether Claude's sandbox `denyWrite` constrains the Write and Edit tools, not only Bash; the spike measured Bash.
- Whether Codex's `workspace-write` sandbox restricts reads (Codex core is not in the adapter package).
- Spec 12's push through askpass with `-c credential.helper=` (still an open question in `open-question.md`) was not examined.
