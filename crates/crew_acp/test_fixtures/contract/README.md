# Adapter initialize contracts

These fixtures contain only the provider label, protocol version, and sanitized agent capabilities. All string values are replaced with `[redacted]`, and `_meta` is removed recursively. Do not copy raw traffic or adapter stderr into this directory.

The integration tests use the ACP 3.4.0 client builder directly. Each adapter runs in a fresh process, accepts no permission requests, and has a 60-second initialize deadline. No sessions or prompts are created. Raw stdin, stdout, and stderr are recorded in a private temporary directory (0700; traffic file 0600 on Unix) and removed when the test ends, including failure. Test errors deliberately omit provider diagnostics because they can include account information.

Ordinary `cargo test -p crew_acp` leaves the three real adapter tests ignored. To run them, set `CREW_CONTRACT=1` and set each `CREW_CONTRACT_{CLAUDE,CODEX,ANTIGRAVITY}_CMD` to a **JSON argv array**, for example:

```sh
export CREW_CONTRACT_CLAUDE_CMD='["/absolute/path/to/node", "/absolute/path/to/claude-agent-acp/dist/index.js"]'
export CREW_CONTRACT_CODEX_CMD='["/absolute/path/to/node", "/absolute/path/to/codex-acp/dist/index.js"]'
export CREW_CONTRACT_ANTIGRAVITY_CMD='["/absolute/path/to/agy_acp_server.par"]'
CREW_CONTRACT=1 cargo test -p crew_acp --test contract -- --ignored --test-threads=1
```

Commands inherit the invoking shell environment; supply authentication there. Never put credentials in command arrays, fixture files, or the repository. Initialization itself does not send a billable prompt; later contract tests may consume provider quota.

Set `CREW_CONTRACT_WRITE_FIXTURES=1` for an explicit refresh of the sanitized initialize fixtures. Review the diff before committing. This switch never exports raw traffic. Every future prompt test must choose its permission policy explicitly; the initialize tests cancel every permission request and advertise no filesystem or terminal capability.

## Verification on 2026-10-25

`cargo test -p crew_acp --target-dir /tmp/crew-acp-contract-target` passed: one fixture-redaction test passed and three adapter tests were ignored. The redaction test was observed failing before implementation with synthetic email and token strings.

With the documented command variables set in the shell, the ignored initialize tests passed for Antigravity, Claude, and Codex in 5.37 seconds. All negotiated ACP protocol version 1. Claude advertised `sessionCapabilities.systemPrompt.mode.override`, which the test checks before exporting its fixture. The three JSON fixtures here are from that run; they contain no raw messages, account identities, or credentials. This initializes adapters only and does not establish that their authentication or prompt execution succeeds.
