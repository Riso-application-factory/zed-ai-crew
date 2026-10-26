# Adapter initialize contracts

These fixtures contain only the provider label, protocol version, and sanitized agent capabilities, taken from the raw initialize response the adapter sent (not a re-serialized typed response, which would add schema defaults and drop fields the schema does not model). All string values are replaced with `[redacted]`, and `_meta` is removed recursively. Do not copy raw traffic or adapter stderr into this directory.

The integration tests use the ACP 3.4.0 client builder directly. Each adapter runs in a fresh process, accepts no permission requests, and has a 60-second initialize deadline. No sessions or prompts are created.

The harness advertises the same client capabilities as Zed's `client_capabilities_for_agent` (`crates/agent_servers/src/acp/v1.rs`): file read and write, terminal, terminal auth, and form and URL elicitation. Adapters change behavior based on these, so advertising less would test behavior Zed never sees. Update `initialize_request` in `tests/support/contract_support.rs` whenever Zed changes that function. The harness serves `fs/read_text_file` and `fs/write_text_file` only inside a `workspace` folder of the test directory (symlinks are resolved first), refuses `terminal/create`, and declines every elicitation. Claude and Codex run with `FileAccess::Forbidden`: any `fs/*` or `terminal/*` request from them fails the test, because Zed applies file writes without a permission step and runs terminals without a sandbox (tech spec 10.4).

Raw stdin, stdout, and stderr are recorded in a private temporary directory (0700; traffic file 0600 on Unix). The directory is removed when a test passes. When a test fails, it is kept and the error names its path; it can contain account details, so delete it after reading. Test errors deliberately omit provider diagnostics for the same reason.

Ordinary `cargo test -p crew_acp` leaves the three real adapter tests ignored. To run them, set `CREW_CONTRACT=1` and set each `CREW_CONTRACT_{CLAUDE,CODEX,ANTIGRAVITY}_CMD` to a **JSON argv array**, for example:

```sh
export CREW_CONTRACT_CLAUDE_CMD='["/absolute/path/to/node", "/absolute/path/to/claude-agent-acp/dist/index.js"]'
export CREW_CONTRACT_CODEX_CMD='["/absolute/path/to/node", "/absolute/path/to/codex-acp/dist/index.js"]'
export CREW_CONTRACT_ANTIGRAVITY_CMD='["/absolute/path/to/agy_acp_server.par"]'
CREW_CONTRACT=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1
```

Commands inherit the invoking shell environment; supply authentication there. Never put credentials in command arrays, fixture files, or the repository. Initialization itself does not send a billable prompt; later contract tests may consume provider quota.

Set `CREW_CONTRACT_WRITE_FIXTURES=1` for an explicit refresh of the sanitized initialize fixtures. Review the diff before committing. This switch never exports raw traffic. Every future prompt test must choose its permission policy and its `FileAccess` explicitly; the initialize tests cancel every permission request.

## Verification on 2026-10-25 and refresh on 2026-10-26

`cargo test -p crew_acp` passed: four offline tests passed (fixture redaction, Zed client capabilities advertisement, forbidden `fs/*`/`terminal/*` check, and workspace path confinement) and three live adapter tests were ignored.

On 2026-10-26, with the documented command variables set in the shell on macOS, `CREW_CONTRACT=1 CREW_CONTRACT_WRITE_FIXTURES=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1` passed for Antigravity (1.4.3), Claude (`claude-agent-acp` 0.95.0), and Codex (`codex-acp` 2.2.2) in 3.18 seconds using the revised v1.2 harness. All negotiated ACP protocol version 1. Claude advertised `sessionCapabilities.systemPrompt.mode.override`, which the test checks before exporting its fixture; Codex's raw response also includes `sessionCapabilities.subagents`, and schema-defaulted booleans absent on the wire (`audio: false` on Claude/Codex, `acp: false` on Antigravity/Claude) are no longer injected. The three JSON fixtures here are from that refreshed run; they contain no raw messages, account identities, or credentials. This initializes adapters only and does not establish that their authentication or prompt execution succeeds.
