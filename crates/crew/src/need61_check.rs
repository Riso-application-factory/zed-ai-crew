use std::{fs, path::Path};

// Split literals so the checker can scan its own definitions and negative fixtures.
const FORBIDDEN: &[&str] = &[
    concat!(".", "claude"),
    concat!(".", "codex"),
    concat!(".", "gemini"),
    concat!("antigravity-acp/", "conversations"),
    concat!("api.", "anthropic.com"),
    concat!("api.", "openai.com"),
    concat!("chatgpt", ".com"),
    concat!("generativelanguage.", "googleapis.com"),
    concat!("Command::new(\"", "claude\")"),
    concat!("Command::new(\"", "codex\")"),
    concat!("app", "-server"),
];

const ALLOWLIST: &[(&str, &str)] = &[];

fn violations(source: &str) -> Vec<&'static str> {
    let source = ALLOWLIST
        .iter()
        .fold(source.to_owned(), |source, (allowed, reason)| {
            assert!(
                !reason.trim().is_empty(),
                "NEED 61 exception requires a reason"
            );
            source.replace(allowed, "")
        });
    let compact: String = source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    FORBIDDEN
        .iter()
        .copied()
        .filter(|pattern| compact.contains(pattern))
        .collect()
}

#[test]
fn need61_rejects_provider_access() {
    for source in [
        concat!("~/.", "codex/config.toml"),
        concat!(".", "claude/settings.json"),
        concat!(".", "gemini/settings.json"),
        concat!("antigravity-acp/", "conversations"),
        concat!("https://api.", "anthropic.com/v1/messages"),
        concat!("https://api.", "openai.com/v1/responses"),
        concat!("https://chatgpt", ".com"),
        concat!("https://generativelanguage.", "googleapis.com"),
        concat!("Command::new(\"", "claude\")"),
        concat!("Command :: new ( \"", "codex\" )"),
        concat!("app", "-server"),
    ] {
        assert!(
            !violations(source).is_empty(),
            "missed forbidden provider access"
        );
    }
}

#[test]
fn need61_allows_acp_registry_ids() {
    assert!(violations("claude-acp codex-acp antigravity-acp").is_empty());
}

#[test]
fn need61_crew_sources_use_only_acp() -> std::io::Result<()> {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| {
            std::io::Error::other("Crew crate must be inside the workspace crates folder")
        })?;
    let mut failures = Vec::new();
    for entry in fs::read_dir(crates)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && entry.file_name().to_string_lossy().starts_with("crew") {
            scan_directory(&entry.path().join("src"), &mut failures)?;
        }
    }
    assert!(
        failures.is_empty(),
        "NEED 61 violations:\n{}",
        failures.join("\n")
    );
    Ok(())
}

fn scan_directory(directory: &Path, failures: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            scan_directory(&path, failures)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = fs::read_to_string(&path)?;
            for violation in violations(&source) {
                failures.push(format!("{}: {violation}", path.display()));
            }
        }
    }
    Ok(())
}
