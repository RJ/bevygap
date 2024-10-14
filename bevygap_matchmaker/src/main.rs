// use std::time::Duration;

use async_nats::jetstream;
// use async_nats::jetstream::stream::StorageType;
use async_nats::Client;
use clap::Parser;
use edgegap::apis::applications_api::*;
use edgegap::apis::configuration::*;
use edgegap::apis::deployments_api::*;
use futures::stream::StreamExt;
use lightyear::connection::netcode::PRIVATE_KEY_BYTES;
use log::*;
use session_service::session_cleanup_supervisor;
use tracing_subscriber::{layer::*, util::*};

use bevygap_shared::*;

mod session_service;

fn edgegap_configuration(_settings: &Settings) -> Configuration {
    let key =
        std::env::var("EDGEGAP_API_KEY").expect("EDGEGAP_API_KEY environment variable is not set");
    Configuration {
        base_path: "https://api.edgegap.com/".to_string(),
        api_key: Some(ApiKey { prefix: None, key }),
        ..Default::default()
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Settings {
    #[arg(long, default_value = "spacepit_server")]
    app_name: String,
    #[arg(long, default_value = "v0.0.1")]
    app_version: String,
    /// private key, in format 1,2,3,4..  which should be 32 u8s long (for signing lightyear tokens)
    #[arg(long, default_value = "")]
    lightyear_private_key: String,
    /// The lightyear protocol id (u64)
    #[arg(long, default_value = "1982")]
    lightyear_protocol_id: u64,
    /// The webhook url for edgegap session creation events
    /// (should write to nats for you, see bevygap_webhook_sink)
    #[arg(long, default_value = None)]
    session_webhook_url: Option<String>,
}

impl Settings {
    pub fn private_key_bytes(&self) -> [u8; PRIVATE_KEY_BYTES] {
        if self.lightyear_private_key.is_empty() {
            return [0u8; PRIVATE_KEY_BYTES];
        }
        let private_key: Vec<u8> = self
            .lightyear_private_key
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == ',')
            .collect::<String>()
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .expect("Failed to parse number in private key")
            })
            .collect();

        if private_key.len() != PRIVATE_KEY_BYTES {
            panic!(
                "Private key must contain exactly {} numbers",
                PRIVATE_KEY_BYTES
            );
        }

        let mut bytes = [0u8; PRIVATE_KEY_BYTES];
        bytes.copy_from_slice(&private_key);
        bytes
    }

    pub fn protocol_id(&self) -> u64 {
        self.lightyear_protocol_id
    }
}

async fn watch_for_gameserver_announcements(
    state: &MatchmakerState,
) -> Result<(), async_nats::Error> {
    info!("Watching for gameserver announcements");
    let client = state.nats_client();
    let mut subscriber = client.subscribe("gameserver.contexts").await?;

    while let Some(message) = subscriber.next().await {
        info!("NEW GAMESERVER: {:?}", message);
    }
    info!("Gameserver announcement watcher exiting");
    Ok(())
}

#[derive(Clone)]
pub(crate) struct MatchmakerState {
    nats: BevygapNats,
    api_config: Configuration,
    settings: Settings,
}

impl MatchmakerState {
    pub(crate) fn nats_client(&self) -> Client {
        self.nats.client()
    }
    pub(crate) fn configuration(&self) -> &Configuration {
        &self.api_config
    }
    pub(crate) fn kv_s2c(&self) -> &jetstream::kv::Store {
        self.nats.kv_s2c()
    }
    pub(crate) fn kv_c2s(&self) -> &jetstream::kv::Store {
        self.nats.kv_c2s()
    }
    pub(crate) fn kv_sessions(&self) -> &jetstream::kv::Store {
        self.nats.kv_sessions()
    }
    pub(crate) fn kv_cert_digests(&self) -> &jetstream::kv::Store {
        self.nats.kv_cert_digests()
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    info!("Starting Edgegap Matchmaker");
    let bgnats = BevygapNats::new_and_connect("matchmaker").await.unwrap();

    let settings = Settings::parse();
    // info!("priv key bytes: {:?}", settings.private_key_bytes());

    let api_config = edgegap_configuration(&settings);

    let mm_state = MatchmakerState {
        nats: bgnats,
        api_config,
        settings,
    };

    // ensure the specified app, version, and deployment are valid and ready for players.
    verify_application(&mm_state).await?;

    let state = mm_state.clone();
    let _watcher = tokio::spawn(async move { session_cleanup_supervisor(&state).await });

    let state = mm_state.clone();
    let _watcher = tokio::spawn(async move {
        match watch_for_gameserver_announcements(&state).await {
            Ok(_) => info!("Gameserver announcement watcher completed"),
            Err(e) => error!("Error in gameserver announcement watcher: {}", e),
        }
    });

    let state = mm_state.clone();
    let session_service = tokio::spawn(async move {
        match session_service::session_request_supervisor(&state).await {
            Ok(_) => info!("Session service completed"),
            Err(e) => error!("Error in session service: {}", e),
        }
    });

    // just to block from exiting:
    session_service.await.unwrap();

    // shouldn't get here
    info!("Edgegap Matchmaker exiting");
    Ok(())
    // dbg!(deployments);
}

async fn verify_application(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let config = state.configuration();
    let settings = &state.settings;

    let app = application_get(config, settings.app_name.as_str())
        .await
        .unwrap_or_else(|e| panic!("Edgegap API doesn't know this application name: {e}"));

    info!(
        "🟢 Application '{}', active: {}, last_updated: {}",
        app.name, app.is_active, app.last_updated
    );

    let app_version = app_version_get(
        config,
        settings.app_name.as_str(),
        settings.app_version.as_str(),
    )
    .await
    .unwrap_or_else(|e| panic!("Edgegap API doesn't know this application version: {e}"));

    if app_version.is_active.unwrap_or(false) {
        info!("🟢 Application version '{}' is active.", app_version.name);
    } else {
        error!(
            "🔴 Application version '{}' is not active, aborting.",
            app_version.name
        );
        std::process::exit(1);
    }

    info!("✅ {} @ {}", settings.app_name, settings.app_version);

    Ok(())
}

// https://fdeantoni.medium.com/from-env-logger-to-tokio-tracing-and-opentelemetry-adb247c0d40f
fn setup_logging() {
    // Set environment for logging configuration
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // Start logging to console
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();
}
