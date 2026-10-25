# Crew hook points verified on the fork

Verified on 2026-10-25 against fork HEAD `11caaae9f733f9a8ae242dd18305d36f244fcb56` and upstream `347f171a3fd00a3b672382d78348b37bd969d17c`. Locations describe the current fork after M0 additions, compared with upstream; later milestone changes must refresh this map. `[Đã kiểm]` means inspected source, not a completed Crew implementation.

Scope: [spec 3.2](tech-spec.md#muc-3-2), [appendix B.1](tech-spec.md#phu-luc-b), and [T0.2](plan/M0-prep-and-spikes.md). Statuses: `khớp` = same location and interface, `dời dòng` = moved location with usable interface, `đổi API` = interface/dependency drift that the consuming task must account for.

## Core edit boundaries

Only H1–H11 authorize core edits. H5 covers `crates/agent_ui`; H6 covers `crates/onboarding`; H7 covers `crates/settings_content` and `crates/settings_ui`; H11 covers authentication flow plus `custom.rs`. New core hooks require a spec and map update in the same change.

## H1–H11

| ID | Symbol / fact | Fork path:line | Upstream path:line | Status | Finding |
| --- | --- | --- | --- | --- | --- |
| H1 | main / askpass::main | `crates/zed/src/main.rs:217`; `crates/askpass/src/askpass.rs:414` | `crates/zed/src/main.rs:217`; `crates/askpass/src/askpass.rs:414` | khớp | Existing askpass forwards the prompt then copies socket to stdout; Crew needs a separate two-way bridge. |
| H2 | AcpConnection::{new,load,resume}_session; mcp_servers_for_project | `crates/agent_servers/src/acp/v1.rs:1238`; `crates/agent_servers/src/acp/v1.rs:1418`; `crates/agent_servers/src/acp/v1.rs:1457`; `crates/agent_servers/src/acp/v1.rs:9134` | `crates/agent_servers/src/acp/v1.rs:1298`; `crates/agent_servers/src/acp/v1.rs:1478`; `crates/agent_servers/src/acp/v1.rs:1517`; `crates/agent_servers/src/acp/v1.rs:9651` | dời dòng | No SessionOptions API yet; caller MCP list and ROLE need the planned opt-in methods. |
| H3 | client_builder / session and elicitation notifications | `crates/agent_servers/src/acp/v1.rs:368`; `crates/agent_servers/src/acp/v1.rs:436`; `crates/agent_servers/src/acp/v1.rs:440` | `crates/agent_servers/src/acp/v1.rs:402`; `crates/agent_servers/src/acp/v1.rs:470`; `crates/agent_servers/src/acp/v1.rs:474` | dời dòng | Builder has moved into client_builder; no auth extension handler or steering wrapper. |
| H4 | handle_session_update; update_usage; TokenUsageUpdated | `crates/acp_thread/src/acp_thread.rs:4404`; `crates/acp_thread/src/acp_thread.rs:5366`; `crates/acp_thread/src/acp_thread.rs:3576` | `crates/acp_thread/src/acp_thread.rs:4739`; `crates/acp_thread/src/acp_thread.rs:5892`; `crates/acp_thread/src/acp_thread.rs:3711` | đổi API | Fork TokenUsageUpdated is a unit variant; upstream carries OutputTokensUpdate. Preserve the fork event shape when adding Crew usage metadata. Unknown v1 updates are still dropped. |
| H5 | Agent::server; render_message_editor; collect_all_sessions | `crates/agent_ui/src/agent_ui.rs:578`; `crates/agent_ui/src/conversation_view/thread_view.rs:4975`; `crates/agent_ui/src/thread_import.rs:794` | `crates/agent_ui/src/agent_ui.rs:579`; `crates/agent_ui/src/conversation_view/thread_view.rs:5372`; `crates/agent_ui/src/thread_import.rs:794` | dời dòng | Factory registry, Crew actions/sidebar, read-only worker view and import ownership filter are future work. |
| H6 | onboarding::init | `crates/onboarding/src/onboarding.rs:72` | `crates/onboarding/src/onboarding.rs:72` | khớp | Existing initialization anchor for the future Crew onboarding page. |
| H7 | ProjectSettingsContent; settings_ui::init | `crates/settings_content/src/project.rs:44`; `crates/settings_ui/src/settings_ui.rs:451` | `crates/settings_content/src/project.rs:44`; `crates/settings_ui/src/settings_ui.rs:451` | khớp | Future crew settings field and Settings Editor page. |
| H8 | CrewFeatureFlag; upstream registration pattern DiffReviewFeatureFlag | `crates/feature_flags/src/flags.rs:123` | `crates/feature_flags/src/flags.rs:30` | dời dòng | Crew flag added by T0.5; enabled_for_staff=false, disabled by default. |
| H9 | zed::initialize_workspace | `crates/zed/src/zed.rs:430` | `crates/zed/src/zed.rs:430` | khớp | T0.5 calls crew_ui::init(cx) synchronously at line 432, after main.rs initializes the command palette. Early zed::init is unsuitable because intervening entity creation flushes deferred callbacks. crates/zed/Cargo.toml declares the required direct crew_ui dependency. |
| H10 | bounded_npm_package_spec; remove_stale_versioned_archive_cache_dirs | `crates/project/src/agent_server_store.rs:1456`; `crates/project/src/agent_server_store.rs:1055`; `crates/project/src/agent_server_store.rs:1309` | `crates/project/src/agent_server_store.rs:1456`; `crates/project/src/agent_server_store.rs:1055`; `crates/project/src/agent_server_store.rs:1309` | khớp | Use a distinct Crew agent ID/cache and an exact package version; normal Zed update policy stays unchanged. |
| H11 | CustomAgentServer env forwarding | `crates/agent_servers/src/custom.rs:243` | `crates/agent_servers/src/custom.rs:243` | khớp | The misspelled OPEN_AI_API_KEY remains. API-key authentication UI and Antigravity login URL fallback are future H11 work. |

## Existing APIs (no core edit required)

| ID | Symbol / fact | Fork path:line | Upstream path:line | Status | Finding |
| --- | --- | --- | --- | --- | --- |
| A1 | AcpThreadEvent::ToolAuthorizationRequested | `crates/acp_thread/src/acp_thread.rs:3579` | `crates/acp_thread/src/acp_thread.rs:3714` | dời dòng |  |
| A2 | authorize_tool_call | `crates/acp_thread/src/acp_thread.rs:6077` | `crates/acp_thread/src/acp_thread.rs:6605` | dời dòng |  |
| A3 | authorize_permission_request | `crates/acp_thread/src/acp_thread.rs:6092` | `crates/acp_thread/src/acp_thread.rs:6620` | dời dòng |  |
| A4 | read_text_file | `crates/acp_thread/src/acp_thread.rs:7250` | `crates/acp_thread/src/acp_thread.rs:7822` | dời dòng | Project path lookup rejects paths outside project worktrees; signature includes reuse_shared_snapshot. |
| A5 | write_text_file | `crates/acp_thread/src/acp_thread.rs:7330` | `crates/acp_thread/src/acp_thread.rs:7902` | dời dòng | Project path lookup rejects paths outside project worktrees. |
| A6 | create_worktree_workspace | `crates/git_ui_core/src/worktree_service.rs:736` | `crates/git_ui_core/src/worktree_service.rs:736` | khớp |  |
| A7 | McpServer / McpServer::new | `crates/context_server/src/listener.rs:34`; `crates/context_server/src/listener.rs:55` | `crates/context_server/src/listener.rs:34`; `crates/context_server/src/listener.rs:55` | khớp |  |
| A8 | McpServer::add_tool | `crates/context_server/src/listener.rs:87` | `crates/context_server/src/listener.rs:87` | khớp |  |
| A9 | McpServerTool | `crates/context_server/src/listener.rs:402` | `crates/context_server/src/listener.rs:402` | khớp |  |
| A10 | db::static_connection! | `crates/db/src/db.rs:247` | `crates/db/src/db.rs:247` | khớp |  |
| A11 | CredentialsProvider | `crates/credentials_provider/src/credentials_provider.rs:11` | `crates/credentials_provider/src/credentials_provider.rs:14` | dời dòng |  |
| A12 | macos_seatbelt::wrap_invocation | `crates/sandbox/src/macos_seatbelt.rs:150` | `crates/sandbox/src/macos_seatbelt.rs:150` | khớp |  |
| A13 | SandboxPolicy | `crates/sandbox/src/sandbox.rs:85` | `crates/sandbox/src/sandbox.rs:85` | khớp |  |

## Appendix B.1 audit

| ID | Symbol / fact | Fork path:line | Upstream path:line | Status | Finding |
| --- | --- | --- | --- | --- | --- |
| B1.01 | Unix socket MCP / add_tool / McpServerTool | `crates/context_server/src/listener.rs:34`; `crates/context_server/src/listener.rs:87`; `crates/context_server/src/listener.rs:402` | `crates/context_server/src/listener.rs:34`; `crates/context_server/src/listener.rs:87`; `crates/context_server/src/listener.rs:402` | khớp |  |
| B1.02 | One-way askpass | `crates/askpass/src/askpass.rs:414`; `crates/zed/src/main.rs:217` | `crates/askpass/src/askpass.rs:414`; `crates/zed/src/main.rs:217` | khớp |  |
| B1.03 | AgentConnection::new_session / into_any | `crates/acp_thread/src/connection.rs:104`; `crates/acp_thread/src/connection.rs:304` | `crates/acp_thread/src/connection.rs:109`; `crates/acp_thread/src/connection.rs:309` | dời dòng |  |
| B1.04 | Project-wide MCP selection | `crates/agent_servers/src/acp/v1.rs:9134` | `crates/agent_servers/src/acp/v1.rs:9651` | dời dòng |  |
| B1.05 | v1 new session / permission / client builder | `crates/agent_servers/src/acp/v1.rs:1238`; `crates/agent_servers/src/acp/v1.rs:9320`; `crates/agent_servers/src/acp/v1.rs:368` | `crates/agent_servers/src/acp/v1.rs:1298`; `crates/agent_servers/src/acp/v1.rs:9837`; `crates/agent_servers/src/acp/v1.rs:402` | dời dòng |  |
| B1.06 | FakeAcpAgentConnection | `crates/agent_servers/src/acp/v1.rs:1841` | `crates/agent_servers/src/acp/v1.rs:1909` | dời dòng |  |
| B1.07 | Agent::server and all three callers | `crates/agent_ui/src/agent_ui.rs:578`; `crates/agent_ui/src/agent_panel.rs:4665`; `crates/agent_ui/src/thread_import.rs:704`; `crates/agent_ui/src/threads_archive_view.rs:832` | `crates/agent_ui/src/agent_ui.rs:579`; `crates/agent_ui/src/agent_panel.rs:4665`; `crates/agent_ui/src/thread_import.rs:704`; `crates/agent_ui/src/threads_archive_view.rs:832` | dời dòng |  |
| B1.08 | Load/resume and idle release | `crates/agent_ui/src/conversation_view.rs:1361`; `crates/agent_ui/src/agent_panel.rs:4313` | `crates/agent_ui/src/conversation_view.rs:1410`; `crates/agent_ui/src/agent_panel.rs:4313` | dời dòng | Reconnect uses the same connection state flow; load falls back to resume. |
| B1.09 | Thread import without cwd filter | `crates/agent_ui/src/thread_import.rs:794` | `crates/agent_ui/src/thread_import.rs:794` | khớp |  |
| B1.10 | ACP file access / project path lookup | `crates/acp_thread/src/acp_thread.rs:7250`; `crates/acp_thread/src/acp_thread.rs:7330`; `crates/project/src/worktree_store.rs:482` | `crates/acp_thread/src/acp_thread.rs:7822`; `crates/acp_thread/src/acp_thread.rs:7902`; `crates/project/src/worktree_store.rs:482` | dời dòng |  |
| B1.11 | Authorization event / responses | `crates/acp_thread/src/acp_thread.rs:3579`; `crates/acp_thread/src/acp_thread.rs:6077`; `crates/acp_thread/src/acp_thread.rs:6092` | `crates/acp_thread/src/acp_thread.rs:3714`; `crates/acp_thread/src/acp_thread.rs:6605`; `crates/acp_thread/src/acp_thread.rs:6620` | dời dòng |  |
| B1.12 | Dropped update / cost.meta-only usage | `crates/acp_thread/src/acp_thread.rs:4404`; `crates/acp_thread/src/acp_thread.rs:5366` | `crates/acp_thread/src/acp_thread.rs:4739`; `crates/acp_thread/src/acp_thread.rs:5892` | đổi API | Top-level usage meta is not persisted; event payload differs as recorded in H4. |
| B1.13 | Subagent composer omission | `crates/agent_ui/src/conversation_view/thread_view.rs:4975` | `crates/agent_ui/src/conversation_view/thread_view.rs:5372` | dời dòng | The is_subagent early return remains; the spec line 1529 is stale. |
| B1.14 | External agent models not enumerated | `crates/agent_ui/src/agent_panel.rs:5008` | `crates/agent_ui/src/agent_panel.rs:5008` | khớp |  |
| B1.15 | Misspelled Codex environment key | `crates/agent_servers/src/custom.rs:243` | `crates/agent_servers/src/custom.rs:243` | khớp |  |
| B1.16 | Worktree workspace | `crates/git_ui_core/src/worktree_service.rs:736` | `crates/git_ui_core/src/worktree_service.rs:736` | khớp |  |
| B1.17 | Seatbelt / policy | `crates/sandbox/src/macos_seatbelt.rs:150`; `crates/sandbox/src/sandbox.rs:85` | `crates/sandbox/src/macos_seatbelt.rs:150`; `crates/sandbox/src/sandbox.rs:85` | khớp |  |
| B1.18 | Keychain / SQLite / feature flag | `crates/credentials_provider/src/credentials_provider.rs:11`; `crates/db/src/db.rs:247`; `crates/feature_flags/src/flags.rs:45` | `crates/credentials_provider/src/credentials_provider.rs:14`; `crates/db/src/db.rs:247`; `crates/feature_flags/src/flags.rs:45` | dời dòng |  |
| B1.19 | Npx range / archive cleanup | `crates/project/src/agent_server_store.rs:1456`; `crates/project/src/agent_server_store.rs:1055` | `crates/project/src/agent_server_store.rs:1456`; `crates/project/src/agent_server_store.rs:1055` | khớp |  |
| B1.20 | Workspace dependencies | `Cargo.toml:544`; `Cargo.toml:692`; `Cargo.toml:872`; `Cargo.toml:818`; `Cargo.toml:897` | `Cargo.toml:533`; `Cargo.toml:680`; `Cargo.toml:878`; `Cargo.toml:824`; `Cargo.toml:903` | đổi API | Fork ACP =3.4.0; upstream =3.5.0. Both enable schemars, unstable and unstable_protocol_v2. |

## Fork differences and outstanding gates

- ACP is pinned to `=3.4.0`, with `schemars`, `unstable`, `unstable_protocol_v2`; upstream pins `=3.5.0` with the same features. The deliberate fork pin is reviewed and accepted for M0; S6 must test this version.
- H4 has real API drift: `TokenUsageUpdated` carries no payload on this fork. Resolved in spec H4 and T1.10/T1.11: retain the unit event and v1 APIs, add a separate usage-details event carrying used/size/meta. Implementation remains M1 work.
- Root `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` are mode `120000` symlinks to `.rules` in both trees. The fork appends one Crew section of at most 10 lines to `.rules`, pointing to [Crew agent instructions](AGENTS.md).
- T0.1 visibility check: `gh repo view Riso-application-factory/zed-ai-crew --json visibility -q .visibility` returned **PUBLIC**, not PRIVATE, on 2026-10-25. NEED 58 remains unsatisfied; repository visibility was not changed.
- Brief v3.5, spec v2.2, overview and M0–M7 sources already exist. [S1–S5](spikes/S1-S5.md) packages the supplied findings; no provider calls were made for this packaging.

## Recheck method

Find each symbol with `rg -n`, inspect the containing function and compare with `git show 347f171:<path>`. The upstream short hash was resolved with `gh api repos/zed-industries/zed/commits/347f171 --jq .sha`, then fetched by full SHA over HTTPS. GitHub checks use the explicit fork name because the CLI default can select upstream. An independent reviewer must reopen five rows before declaring T0.2 accepted.
