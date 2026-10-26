#[path = "support/contract_support.rs"]
mod support;

use agent_client_protocol::LineDirection;
use anyhow::Context as _;

#[test]
fn fixtures_redact_nested_account_and_credential_values() {
    let raw = serde_json::json!({"capability": true, "nested": ["person@example.test", {"accessToken": "synthetic-token"}]});
    assert_eq!(
        support::redact_fixture(raw),
        serde_json::json!({"capability": true, "nested": ["[redacted]", {"accessToken": "[redacted]"}]})
    );
}

#[test]
fn initialize_advertises_zed_client_capabilities() -> anyhow::Result<()> {
    let request = serde_json::to_value(support::initialize_request())?;
    let capabilities = request
        .get("clientCapabilities")
        .context("initialize request lacks clientCapabilities")?;
    for pointer in [
        "/fs/readTextFile",
        "/fs/writeTextFile",
        "/terminal",
        "/auth/terminal",
    ] {
        assert_eq!(
            capabilities.pointer(pointer),
            Some(&serde_json::Value::Bool(true)),
            "{pointer}"
        );
    }
    for pointer in ["/elicitation/form", "/elicitation/url"] {
        assert!(
            capabilities
                .pointer(pointer)
                .is_some_and(serde_json::Value::is_object),
            "{pointer}"
        );
    }
    Ok(())
}

#[test]
fn fs_and_terminal_requests_from_the_agent_fail_the_shared_check() {
    let traffic = [
        (
            LineDirection::Stdout,
            r#"{"jsonrpc":"2.0","id":3,"method":"fs/write_text_file","params":{}}"#,
        ),
        (
            LineDirection::Stdout,
            r#"{"jsonrpc":"2.0","id":4,"method":"terminal/create","params":{}}"#,
        ),
        (
            LineDirection::Stdout,
            r#"{"jsonrpc":"2.0","method":"session/update","params":{}}"#,
        ),
        (
            LineDirection::Stdin,
            r#"{"jsonrpc":"2.0","id":5,"method":"fs/read_text_file","params":{}}"#,
        ),
        (LineDirection::Stderr, "fs/write_text_file is not JSON"),
    ];
    assert_eq!(
        support::forbidden_client_methods(traffic),
        vec!["fs/write_text_file", "terminal/create"]
    );
}

#[test]
fn file_requests_stay_inside_the_test_workspace() -> anyhow::Result<()> {
    let workspace = tempfile::tempdir()?;
    let root = workspace.path().canonicalize()?;
    let outside = tempfile::tempdir()?;
    let secret = outside.path().join("secret.txt");
    std::fs::write(&secret, "synthetic")?;
    assert!(support::confined_path(&root, &root.join("notes.txt")).is_ok());
    assert!(support::confined_path(&root, &secret).is_err());
    assert!(support::confined_path(&root, &root.join("../secret.txt")).is_err());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&secret, root.join("link.txt"))?;
        assert!(support::confined_path(&root, &root.join("link.txt")).is_err());
    }
    Ok(())
}

#[test]
#[ignore = "requires CREW_CONTRACT=1 and a logged-in Claude ACP adapter"]
fn initialize_claude() -> anyhow::Result<()> {
    support::initialize_adapter(
        "claude",
        "CREW_CONTRACT_CLAUDE_CMD",
        true,
        support::FileAccess::Forbidden,
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
        support::FileAccess::Forbidden,
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
        support::FileAccess::TestWorkspace,
        support::deny_all_permissions,
    )
}
