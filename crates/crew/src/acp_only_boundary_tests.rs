use regex::Regex;
use std::{fs, path::Path};

struct Exception {
    file: &'static str,
    line: &'static str,
    reason: &'static str,
}

// An exception covers one file and only the lines that contain its exact text, so one entry
// cannot hide a violation elsewhere.
const EXCEPTIONS: &[Exception] = &[];

// This file is scanned too. Escaped patterns cannot match their own source because of the
// backslashes; plain-text patterns are split for the same reason.
fn forbidden_patterns() -> Result<Vec<(&'static str, Regex)>, regex::Error> {
    [
        (
            "provider home directory",
            r#"(?:^|["'/\\~])\.(?:claude|codex|gemini)(?:[^A-Za-z0-9_]|$)"#,
        ),
        (
            "Antigravity session data",
            concat!("antigravity-acp/", "conversations"),
        ),
        (
            "provider API host",
            r"api\.anthropic\.com|api\.openai\.com|chatgpt\.com|generativelanguage\.googleapis\.com",
        ),
        (
            "provider CLI process",
            r#"(?:Command[ \t]*::[ \t]*new|[A-Za-z0-9_]*_command)[ \t]*\([ \t]*"(?:[^"]*/)?(?:claude|codex|gemini|antigravity|agy)(?:\.exe)?""#,
        ),
        ("Codex app server", concat!("app", "-server")),
    ]
    .into_iter()
    .map(|(label, pattern)| Ok((label, Regex::new(pattern)?)))
    .collect()
}

fn violations(
    file: &str,
    source: &str,
    exceptions: &[Exception],
) -> Result<Vec<String>, regex::Error> {
    let patterns = forbidden_patterns()?;
    let mut found = Vec::new();
    for (index, line) in source.lines().enumerate() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        if exceptions
            .iter()
            .any(|exception| file.ends_with(exception.file) && line.contains(exception.line))
        {
            continue;
        }
        for (label, pattern) in &patterns {
            if pattern.is_match(line) {
                found.push(format!("{file}:{}: {label}", index + 1));
            }
        }
    }
    Ok(found)
}

#[test]
fn need61_rejects_provider_access() -> Result<(), regex::Error> {
    for source in [
        concat!("let config = \"~/.", "codex/config.toml\";"),
        concat!("let settings = home.join(\".", "claude\");"),
        concat!("let state = \"/Users/someone/.", "claude.json\";"),
        concat!("let settings = Path::new(\".", "gemini/settings.json\");"),
        concat!("let sessions = \"antigravity-acp/", "conversations\";"),
        concat!("let url = \"https://api.", "anthropic.com/v1/messages\";"),
        concat!("let url = \"https://api.", "openai.com/v1/responses\";"),
        concat!("let url = \"https://chatgpt", ".com\";"),
        concat!(
            "let url = \"https://generativelanguage.",
            "googleapis.com\";"
        ),
        concat!("let child = Command::new(\"", "claude\");"),
        concat!("let child = Command :: new ( \"", "codex\" );"),
        concat!("let child = smol::process::Command::new(\"", "gemini\");"),
        concat!("let child = Command::new(\"/usr/local/bin/", "claude\");"),
        concat!(
            "let child = util::command::new_command(\"",
            "claude\").arg(\"-p\");"
        ),
        concat!("let child = new_std_command(\"", "codex\");"),
        concat!("let arguments = [\"app", "-server\"];"),
    ] {
        assert!(
            !violations("sample.rs", source, &[])?.is_empty(),
            "missed forbidden provider access in {source}"
        );
    }
    Ok(())
}

#[test]
fn need61_allows_registry_ids_fields_and_comments() -> Result<(), regex::Error> {
    for source in [
        "let ids = [\"claude-acp\", \"codex-acp\", \"antigravity-acp\"];",
        "let note = \"registry ids, e.g. claude-acp and codex-acp\";",
        "let model = profiles.claude.model;",
        "let left = quota.codex();",
        "let key = settings.gemini_key;",
        concat!(
            "/// Crew never reads ~/.",
            "claude or spawns the provider CLI."
        ),
    ] {
        assert_eq!(
            violations("sample.rs", source, &[])?,
            Vec::<String>::new(),
            "{source}"
        );
    }
    Ok(())
}

#[test]
fn need61_exception_covers_one_file_and_line() -> Result<(), regex::Error> {
    let exceptions = [Exception {
        file: "crew_acp/src/sample.rs",
        line: "let documented",
        reason: "test of exception scope",
    }];
    let source = concat!(
        "let documented = \"~/.",
        "codex\";\n",
        "let hidden = \"~/.",
        "codex\";\n",
    );
    assert_eq!(
        violations("crew_acp/src/sample.rs", source, &exceptions)?,
        vec!["crew_acp/src/sample.rs:2: provider home directory"]
    );
    assert_eq!(
        violations("crew_ui/src/sample.rs", source, &exceptions)?.len(),
        2
    );
    Ok(())
}

#[test]
fn need61_crew_sources_use_only_acp() -> Result<(), Box<dyn std::error::Error>> {
    for exception in EXCEPTIONS {
        assert!(
            !exception.reason.trim().is_empty() && !exception.line.trim().is_empty(),
            "NEED 61 exception for {} needs a reason and the exact line text",
            exception.file
        );
    }
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Crew crate must be inside the workspace crates folder")?;
    let mut failures = Vec::new();
    for entry in fs::read_dir(crates)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && entry.file_name().to_string_lossy().starts_with("crew") {
            scan_directory(crates, &entry.path().join("src"), &mut failures)?;
        }
    }
    assert!(
        failures.is_empty(),
        "NEED 61 violations:\n{}",
        failures.join("\n")
    );
    Ok(())
}

fn scan_directory(
    crates: &Path,
    directory: &Path,
    failures: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            scan_directory(crates, &path, failures)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let file = path.strip_prefix(crates)?.to_string_lossy().into_owned();
            failures.extend(violations(&file, &fs::read_to_string(&path)?, EXCEPTIONS)?);
        }
    }
    Ok(())
}
