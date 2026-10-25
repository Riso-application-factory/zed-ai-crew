use agent_client_protocol::schema::{
    ProtocolVersion,
    v1::{
        InitializeRequest, RequestPermissionOutcome, RequestPermissionRequest,
        RequestPermissionResponse,
    },
};
use agent_client_protocol::{AcpAgent, AcpAgentConfig, Client};
use anyhow::{Context as _, bail};
use std::{
    fs::OpenOptions,
    io::Write as _,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

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

pub fn initialize_adapter(
    provider: &str,
    command_variable: &str,
    requires_system_prompt: bool,
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
    let directory = tempfile::Builder::new()
        .prefix("crew-contract-")
        .tempdir()?;
    let traffic = private_file(&directory.path().join("traffic.jsonl"))?;
    let traffic = Arc::new(Mutex::new((traffic, false)));
    let traffic_for_callback = traffic.clone();
    let adapter = AcpAgent::new(AcpAgentConfig::new(program).args(arguments.iter().cloned()))
        .with_debug(move |line, direction| match traffic_for_callback.lock() {
            Ok(mut state) => {
                if writeln!(
                    state.0,
                    "{}",
                    serde_json::json!({"direction": format!("{direction:?}"), "line": line})
                )
                .is_err()
                {
                    state.1 = true;
                }
            }
            Err(_) => eprintln!("contract traffic recorder lock failed"),
        });
    let response = smol::block_on(async {
        smol::future::race(
            async {
                Client.builder()
                    .on_receive_request(
                        async move |request: RequestPermissionRequest, responder, _connection| {
                            responder.respond(permission_policy(request))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .connect_with(adapter, async |connection| {
                        connection.send_request(InitializeRequest::new(ProtocolVersion::V1))
                            .block_task().await
                    }).await.map_err(|error| {
                        let code: i32 = error.code.into();
                        anyhow::anyhow!("{provider}: ACP initialize or process transport failed (code {code}); raw diagnostics were confined to the private temporary directory")
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
    let mut traffic = traffic
        .lock()
        .map_err(|_| anyhow::anyhow!("contract traffic recorder lock failed"))?;
    traffic.0.flush()?;
    anyhow::ensure!(!traffic.1, "contract traffic could not be recorded");
    let response = serde_json::to_value(response)?;
    let capabilities = response
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
    let fixture = serde_json::json!({"provider": provider, "protocolVersion": response.get("protocolVersion"), "agentCapabilities": redact_fixture(capabilities.clone())});
    println!("{}", serde_json::to_string(&fixture)?);
    if std::env::var("CREW_CONTRACT_WRITE_FIXTURES").as_deref() == Ok("1") {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_fixtures/contract");
        std::fs::create_dir_all(&directory)?;
        std::fs::write(
            directory.join(format!("{provider}-initialize.json")),
            format!("{}\n", serde_json::to_string_pretty(&fixture)?),
        )?;
    }
    Ok(())
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
