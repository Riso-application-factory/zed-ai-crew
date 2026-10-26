use agent_client_protocol::schema::{
    ProtocolVersion,
    v1::{
        AuthCapabilities, BooleanConfigOptionCapabilities, ClientCapabilities,
        ClientSessionCapabilities, CompactionCapabilities, CreateElicitationRequest,
        CreateElicitationResponse, CreateTerminalRequest, ElicitationAction,
        ElicitationCapabilities, ElicitationFormCapabilities, ElicitationUrlCapabilities,
        FileSystemCapabilities, InitializeRequest, Meta, NoticeCapabilities, ReadTextFileRequest,
        ReadTextFileResponse, RequestPermissionOutcome, RequestPermissionRequest,
        RequestPermissionResponse, SessionConfigOptionsCapabilities, WriteTextFileRequest,
        WriteTextFileResponse,
    },
};
use agent_client_protocol::{AcpAgent, AcpAgentConfig, Client, Error, LineDirection};
use anyhow::{Context as _, bail};
use std::{
    fs::OpenOptions,
    io::Write as _,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileAccess {
    /// Zed applies `fs/write_text_file` without a permission step and runs `terminal/create`
    /// without a sandbox, so an adapter update that starts sending either must fail the test.
    Forbidden,
    /// Antigravity writes files through `fs/*`, so the harness serves those inside the test
    /// workspace.
    TestWorkspace,
}

struct Recorder {
    file: std::fs::File,
    write_failed: bool,
    lines: Vec<(LineDirection, String)>,
}

pub fn redact_fixture(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(_) => serde_json::Value::String("[redacted]".into()),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.into_iter().map(redact_fixture).collect())
        }
        serde_json::Value::Object(values) => serde_json::Value::Object(
            values
                .into_iter()
                .filter(|(key, _)| key != "_meta")
                .map(|(key, value)| (key, redact_fixture(value)))
                .collect(),
        ),
        value => value,
    }
}

/// Adapters change behavior based on the client capabilities (Claude keeps `AskUserQuestion` only
/// when the client supports form elicitation), so this must advertise exactly what
/// `client_capabilities_for_agent` in `crates/agent_servers/src/acp/v1.rs` advertises.
pub fn initialize_request() -> InitializeRequest {
    let meta = Meta::from_iter([
        ("terminal_output".into(), true.into()),
        ("terminal-auth".into(), true.into()),
    ]);
    let session = ClientSessionCapabilities::new()
        .config_options(
            SessionConfigOptionsCapabilities::new().boolean(BooleanConfigOptionCapabilities::new()),
        )
        .compaction(CompactionCapabilities::new())
        .notices(NoticeCapabilities::new());
    let capabilities = ClientCapabilities::new()
        .fs(FileSystemCapabilities::new()
            .read_text_file(true)
            .write_text_file(true))
        .terminal(true)
        .auth(AuthCapabilities::new().terminal(true))
        .session(session)
        .elicitation(
            ElicitationCapabilities::new()
                .form(ElicitationFormCapabilities::new())
                .url(ElicitationUrlCapabilities::new()),
        )
        .meta(meta);
    InitializeRequest::new(ProtocolVersion::V1).client_capabilities(capabilities)
}

pub fn forbidden_client_methods<'a>(
    lines: impl IntoIterator<Item = (LineDirection, &'a str)>,
) -> Vec<String> {
    lines
        .into_iter()
        .filter(|(direction, _)| *direction == LineDirection::Stdout)
        .filter_map(|(_, line)| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter_map(|message| Some(message.get("method")?.as_str()?.to_owned()))
        .filter(|method| method.starts_with("fs/") || method.starts_with("terminal/"))
        .collect()
}

/// `root` must be canonical. Symlinks are resolved before the check, so a link inside the
/// workspace that points outside it is refused.
pub fn confined_path(root: &Path, requested: &Path) -> Result<PathBuf, Error> {
    let outside = || {
        Error::invalid_params().data(serde_json::json!("path is outside the contract workspace"))
    };
    let parent = requested.parent().ok_or_else(outside)?;
    let file_name = requested.file_name().ok_or_else(outside)?;
    let path = parent
        .canonicalize()
        .map_err(|_| outside())?
        .join(file_name);
    let path = path.canonicalize().unwrap_or(path);
    if path.starts_with(root) {
        Ok(path)
    } else {
        Err(outside())
    }
}

fn read_text_file(
    root: &Path,
    request: &ReadTextFileRequest,
) -> Result<ReadTextFileResponse, Error> {
    let path = confined_path(root, &request.path)?;
    let content = std::fs::read_to_string(path).map_err(Error::into_internal_error)?;
    let content = if request.line.is_none() && request.limit.is_none() {
        content
    } else {
        let start = request
            .line
            .map_or(0, |line| line.saturating_sub(1) as usize);
        let limit = request.limit.map_or(usize::MAX, |limit| limit as usize);
        content
            .lines()
            .skip(start)
            .take(limit)
            .collect::<Vec<_>>()
            .join("\n")
    };
    Ok(ReadTextFileResponse::new(content))
}

fn write_text_file(
    root: &Path,
    request: &WriteTextFileRequest,
) -> Result<WriteTextFileResponse, Error> {
    let path = confined_path(root, &request.path)?;
    std::fs::write(path, &request.content).map_err(Error::into_internal_error)?;
    Ok(WriteTextFileResponse::default())
}

/// The raw `result` of the initialize response. Re-serializing the typed response would add
/// schema defaults and drop fields the schema does not model, so fixtures could not show drift.
fn initialize_result(lines: &[(LineDirection, String)]) -> Option<serde_json::Value> {
    lines
        .iter()
        .filter(|(direction, _)| *direction == LineDirection::Stdout)
        .filter_map(|(_, line)| serde_json::from_str::<serde_json::Value>(line).ok())
        .find_map(|message| {
            let result = message.get("result")?;
            result
                .get("agentCapabilities")
                .is_some()
                .then(|| result.clone())
        })
}

pub fn initialize_adapter(
    provider: &str,
    command_variable: &str,
    requires_system_prompt: bool,
    file_access: FileAccess,
    permission_policy: fn(RequestPermissionRequest) -> RequestPermissionResponse,
) -> anyhow::Result<()> {
    if std::env::var("CREW_CONTRACT").as_deref() != Ok("1") {
        bail!("real adapter tests require CREW_CONTRACT=1");
    }
    let command = std::env::var(command_variable)
        .with_context(|| format!("set {command_variable} to a JSON argv array"))?;
    let arguments: Vec<String> = serde_json::from_str(&command)
        .map_err(|_| anyhow::anyhow!("{command_variable} must be a JSON argv array"))?;
    let (program, arguments) = arguments
        .split_first()
        .context("adapter command must not be empty")?;
    let mut builder = tempfile::Builder::new();
    builder.prefix("crew-contract-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        builder.permissions(std::fs::Permissions::from_mode(0o700));
    }
    let directory = builder.tempdir()?;
    match run_initialize(
        provider,
        program,
        arguments,
        directory.path(),
        requires_system_prompt,
        file_access,
        permission_policy,
    ) {
        Ok(fixture) => {
            println!("{}", serde_json::to_string(&fixture)?);
            if std::env::var("CREW_CONTRACT_WRITE_FIXTURES").as_deref() == Ok("1") {
                let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_fixtures/contract");
                std::fs::create_dir_all(&fixtures)?;
                std::fs::write(
                    fixtures.join(format!("{provider}-initialize.json")),
                    format!("{}\n", serde_json::to_string_pretty(&fixture)?),
                )?;
            }
            Ok(())
        }
        Err(error) => {
            let kept = directory.keep();
            Err(error.context(format!(
                "{provider}: raw traffic kept in {} for diagnosis; it can contain account details, so delete it after reading",
                kept.display()
            )))
        }
    }
}

fn run_initialize(
    provider: &str,
    program: &str,
    arguments: &[String],
    directory: &Path,
    requires_system_prompt: bool,
    file_access: FileAccess,
    permission_policy: fn(RequestPermissionRequest) -> RequestPermissionResponse,
) -> anyhow::Result<serde_json::Value> {
    let workspace = directory.join("workspace");
    std::fs::create_dir(&workspace)?;
    let workspace = workspace.canonicalize()?;
    let recorder = Arc::new(Mutex::new(Recorder {
        file: private_file(&directory.join("traffic.jsonl"))?,
        write_failed: false,
        lines: Vec::new(),
    }));
    let recorder_for_callback = recorder.clone();
    let adapter = AcpAgent::new(AcpAgentConfig::new(program).args(arguments.iter().cloned()))
        .with_debug(move |line, direction| match recorder_for_callback.lock() {
            Ok(mut recorder) => {
                let entry =
                    serde_json::json!({"direction": format!("{direction:?}"), "line": line});
                if writeln!(recorder.file, "{entry}").is_err() {
                    recorder.write_failed = true;
                }
                recorder.lines.push((direction, line.to_owned()));
            }
            Err(_) => eprintln!("contract traffic recorder lock failed"),
        });
    let workspace_for_reads = workspace.clone();
    let workspace_for_writes = workspace;
    smol::block_on(async {
        smol::future::race(
            async {
                Client.builder()
                    .on_receive_request(
                        async move |request: RequestPermissionRequest, responder, _connection| {
                            responder.respond(permission_policy(request))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: ReadTextFileRequest, responder, _connection| {
                            responder.respond_with_result(read_text_file(&workspace_for_reads, &request))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: WriteTextFileRequest, responder, _connection| {
                            responder.respond_with_result(write_text_file(&workspace_for_writes, &request))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |_request: CreateTerminalRequest, responder, _connection| {
                            responder.respond_with_error(Error::invalid_request().data(
                                serde_json::json!("terminals are not available in contract tests"),
                            ))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |_request: CreateElicitationRequest, responder, _connection| {
                            responder.respond(CreateElicitationResponse::new(ElicitationAction::Decline))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .connect_with(adapter, async |connection| {
                        connection.send_request(initialize_request())
                            .block_task().await
                    }).await.map_err(|error| {
                        let code: i32 = error.code.into();
                        anyhow::anyhow!("{provider}: ACP initialize or process transport failed (code {code})")
                    })
            },
            async {
                #[expect(
                    clippy::disallowed_methods,
                    reason = "External ACP processes need a wall-clock deadline without a GPUI dispatcher"
                )]
                let deadline = smol::Timer::after(Duration::from_secs(60));
                deadline.await;
                bail!("{provider}: initialize timed out after 60 seconds")
            },
        ).await
    })?;
    let mut recorder = recorder
        .lock()
        .map_err(|_| anyhow::anyhow!("contract traffic recorder lock failed"))?;
    recorder.file.flush()?;
    anyhow::ensure!(
        !recorder.write_failed,
        "contract traffic could not be recorded"
    );
    if file_access == FileAccess::Forbidden {
        let forbidden = forbidden_client_methods(
            recorder
                .lines
                .iter()
                .map(|(direction, line)| (*direction, line.as_str())),
        );
        anyhow::ensure!(
            forbidden.is_empty(),
            "{provider} sent client requests that Zed serves without a permission step or sandbox: {forbidden:?}"
        );
    }
    let result = initialize_result(&recorder.lines).context("adapter sent no initialize result")?;
    let capabilities = result
        .get("agentCapabilities")
        .context("initialize response lacks agentCapabilities")?;
    if requires_system_prompt {
        anyhow::ensure!(
            capabilities
                .pointer("/sessionCapabilities/systemPrompt")
                .is_some(),
            "Claude lacks sessionCapabilities.systemPrompt"
        );
    }
    Ok(serde_json::json!({
        "provider": provider,
        "protocolVersion": result.get("protocolVersion"),
        "agentCapabilities": redact_fixture(capabilities.clone()),
    }))
}

fn private_file(path: &Path) -> std::io::Result<std::fs::File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    options.open(path)
}

pub fn deny_all_permissions(_request: RequestPermissionRequest) -> RequestPermissionResponse {
    RequestPermissionResponse::new(RequestPermissionOutcome::Cancelled)
}
