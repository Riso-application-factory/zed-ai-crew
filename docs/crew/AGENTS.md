# Crew agent instructions

Read this before implementing a Crew task. The [brief v3.5](brief.md) is the requirement authority; the [tech spec v2.3](tech-spec.md) defines implementation decisions. The [overview](plan/00-overview.md) defines workflow, crate boundaries and completion gates. Read your task in `plan/M0-prep-and-spikes.md` through `plan/M7-model-and-retro.md`, then only the spec sections it cites. Do not invent requirements.

## Commands

Run from the repository root on macOS:

```sh
cargo check -p crew -p crew_acp -p crew_mcp -p crew_github -p crew_models -p crew_ui
cargo test -p <changed_crate>
./script/clippy
cargo fmt --all -- --check
cargo run
cargo test -p crew need61
CREW_CONTRACT=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1
```

Use the actual changed crate name in place of `<changed_crate>`. Use `./script/clippy`, not `cargo clippy`. `cargo run` launches the debug app for UI checks. In the debug app, enable Crew in user settings with `"feature_flags": { "crew": "on" }`. The staff default must remain false; ordinary release builds remain disabled until T7.8. `crew_ui::init` runs in `crates/zed/src/main.rs` right after `agent_ui::init` (H9). Keep it after `command_palette::init`, which resets the command-palette filter Crew uses to hide its actions; `zed::init` runs before that reset, so do not move it there.

Contract tests invoke real ACP adapters and consume provider quota. Run them only when a task requires them, when changing an adapter pin, or before manual testing, with Human available for login. They are ignored and explicitly gated by `CREW_CONTRACT=1`; normal tests must not contact providers. Follow the harness instructions for adapter executable configuration. Never inspect provider credential files to make a test pass.

## Boundaries

- Core edits are limited to [the verified hook map](hook-points.md): H1 and H9 `crates/zed/src/main.rs`; H2/H3 `crates/agent_servers/src/acp/v1.rs`; H4 `crates/acp_thread/src/acp_thread.rs` (H4b may need `v1.rs`); H5 `crates/agent_ui` (H5c also `crates/sidebar`); H6 `crates/onboarding`; H7 `crates/settings_content` and `crates/settings_ui`; H8 `crates/feature_flags/src/flags.rs`; H10 `crates/project/src/agent_server_store.rs`; H11 `crates/agent_servers/src/custom.rs` and the Zed authentication flow; H12 the `fs/*` handlers in `crates/agent_servers/src/acp/v1.rs` and tool-call locations in `crates/acp_thread/src/acp_thread.rs`. Your task's allowed-file list further narrows these paths. New hooks require a map and spec update in the same PR.
- NEED 61: interact with providers only through ACP. Do not read or write provider home folders, invoke their private APIs, or control their CLI outside an ACP adapter. Zed's adapter registry manages adapter installation. Crew must not inspect provider session storage or delete it outside ACP.
- Never put experimental code on `main`. Use a task branch `crew/<task-id>-<short-name>`. Spike code belongs on `spike/<id>-<short-name>` and is never merged into `main`; merge only its report and affected documentation.
- Never put secrets, emails, account IDs or raw provider logs in code, fixtures or PRs. Use fake secret files for sandbox experiments and redact provider output before saving it.
- Gate all Crew behavior behind `CrewFeatureFlag`, including actions, adapter processes and database writes. With the flag off, the fork behaves like upstream. H11 fixes to Zed authentication are the documented exception.
- Preserve the manual-review notice at the start of root `README.md`. Before modifying source, ensure the two required notice lines exist as mandated by root `.rules`.

## Dependency direction

```text
zed -> crew_ui -> crew
crew -> crew_acp -> agent_servers, acp_thread, project
crew -> crew_mcp -> context_server
crew -> crew_github -> http_client, credentials_provider, git
crew -> crew_models -> http_client
zed -> crew_mcp (H1 bridge only)
agent_ui -> agent_servers
```

`agent_ui`, `sidebar`, `agent_servers` and `acp_thread` must not depend on Crew crates. Core integration uses a registry in `agent_servers` populated by Crew. `crew_acp` must not depend on `crew`; define store-access traits in `crew_acp` and implement them in `crew`. `crew_mcp` owns the server, schemas and bridge; concrete Crew tools belong in `crew`.

## Task workflow and completion

1. Work on a task branch from fork `main`. Read the task goal, dependencies, allowed files, acceptance criteria and verification commands.
2. Write meaningful acceptance tests before implementation and observe the expected failure. For documentation-only tasks, inspect source references, anchors and relative links instead.
3. Implement only the authorized task. Report scope conflicts; do not expand requirements. Keep Markdown working documents in English and matching human HTML in Vietnamese, updating both together.
4. Run task verification plus tests for changed crates, the clippy script, formatting, and NEED 61 once available. For UI changes, run the real app and capture evidence.
5. Prepare a PR to fork `main` with task ID, changes, exact commands/results and unverified items. Follow root PR hygiene, including the final `Release Notes:` section. Use GitHub PR reviews with review threads, not issue comments.
6. A different agent in a fresh context independently runs verification and checks the task against the brief/spec and diff. It returns pass or fail with evidence. Both automated and independent gates must pass before merge; Human decides product questions.

Done means all acceptance criteria have evidence, required checks pass, core changes remain inside the hook map, flag-off behavior is preserved, fixtures are sanitized, UI evidence exists where relevant, and independent verification passes. Do not report an unmet gate as complete. In particular, NEED 58 requires a PRIVATE fork; the T0.1 check currently reports PUBLIC, as recorded in the hook map.

## Evidence labels

Use the existing source labels in reports: **[Đã kiểm]** for inspected source/original documents; **[Spike]** for a measured spike result with date and environment; **[Baseline]** for Human's provider baseline; **[Nhớ]** for recalled information not rechecked; **[Suy luận]** for an inference requiring verification. Distinguish inherited spike findings from measurements made in your session. Record real-provider prompt usage in each spike report.
