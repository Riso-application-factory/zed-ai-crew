#[path = "support/contract_support.rs"]
mod support;

#[test]
fn fixtures_redact_nested_account_and_credential_values() {
    let raw = serde_json::json!({"capability": true, "nested": ["person@example.test", {"accessToken": "synthetic-token"}]});
    assert_eq!(
        support::redact_fixture(raw),
        serde_json::json!({"capability": true, "nested": ["[redacted]", {"accessToken": "[redacted]"}]})
    );
}

#[test]
#[ignore = "requires CREW_CONTRACT=1 and a logged-in Claude ACP adapter"]
fn initialize_claude() -> anyhow::Result<()> {
    support::initialize_adapter(
        "claude",
        "CREW_CONTRACT_CLAUDE_CMD",
        true,
        support::deny_all_permissions,
    )
}

#[test]
#[ignore = "requires CREW_CONTRACT=1 and a logged-in Codex ACP adapter"]
fn initialize_codex() -> anyhow::Result<()> {
    support::initialize_adapter(
        "codex",
        "CREW_CONTRACT_CODEX_CMD",
        false,
        support::deny_all_permissions,
    )
}

#[test]
#[ignore = "requires CREW_CONTRACT=1 and a logged-in Antigravity ACP adapter"]
fn initialize_antigravity() -> anyhow::Result<()> {
    support::initialize_adapter(
        "antigravity",
        "CREW_CONTRACT_ANTIGRAVITY_CMD",
        false,
        support::deny_all_permissions,
    )
}
