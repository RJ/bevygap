use std::time::Duration;

use async_nats::jetstream;
use async_nats::jetstream::stream::StorageType;
use async_nats::Client;
use clap::Parser;
use edgegap::apis::applications_api::*;
use edgegap::apis::configuration::*;
use edgegap::apis::deployments_api::*;
use futures::stream::StreamExt;
use lightyear::connection::netcode::PRIVATE_KEY_BYTES;
use log::*;
use tracing_subscriber::{layer::*, util::*};
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
    /// The deployment ID from edgegap
    #[arg(long)]
    deployment_id: String,
    /// private key, in format 1,2,3,4..  which should be 32 u8s long (for signing lightyear tokens)
    #[arg(long, default_value = "")]
    private_key: String,
    /// The lightyear protocol id (u64)
    #[arg(long, default_value = "1982")]
    protocol_id: u64,
}

impl Settings {
    pub fn private_key_bytes(&self) -> [u8; PRIVATE_KEY_BYTES] {
        if self.private_key.is_empty() {
            return [0u8; PRIVATE_KEY_BYTES];
        }
        let private_key: Vec<u8> = self
            .private_key
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
        self.protocol_id
    }
}

async fn connect_to_nats() -> Result<Client, async_nats::Error> {
    info!("Setting up NATS");
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    info!("NATS_URL: {}", nats_url);
    // Connect to NATS.
    let client = async_nats::connect(nats_url).await?;

    Ok(client)
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
    nats_client: Client,
    api_config: Configuration,
    settings: Settings,
    kv_s2c: jetstream::kv::Store,
    kv_c2s: jetstream::kv::Store,
}

impl MatchmakerState {
    pub(crate) fn nats_client(&self) -> Client {
        self.nats_client.clone()
    }
    pub(crate) fn configuration(&self) -> &Configuration {
        &self.api_config
    }
    pub(crate) fn kv_s2c(&self) -> &jetstream::kv::Store {
        &self.kv_s2c
    }
    pub(crate) fn kv_c2s(&self) -> &jetstream::kv::Store {
        &self.kv_c2s
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    info!("Starting Edgegap Matchmaker");
    let nats_client = connect_to_nats().await.unwrap();

    let settings = Settings::parse();

    let api_config = edgegap_configuration(&settings);

    let (kv_s2c, kv_c2s) = create_kv_buckets_for_session_mappings(nats_client.clone()).await?;

    let mm_state = MatchmakerState {
        nats_client: nats_client.clone(),
        api_config,
        settings,
        kv_s2c,
        kv_c2s,
    };

    // ensure the specified app, version, and deployment are valid and ready for players.
    verify_deployment(&mm_state).await?;

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

/// Creates two buckets for mapping between LY client ids and Edgegap session tokens
async fn create_kv_buckets_for_session_mappings(
    client: Client,
) -> Result<(jetstream::kv::Store, jetstream::kv::Store), async_nats::Error> {
    let jetstream = jetstream::new(client);

    let kv_s2c = jetstream
        .create_key_value(async_nats::jetstream::kv::Config {
            bucket: "sessions_eg2ly".to_string(),
            description: "Maps Edgegap Session IDs to Lightyear Client IDs".to_string(),
            max_value_size: 1024,
            // shouldn't need long for the client to receive token, and make connection to gameserver.
            max_age: Duration::from_millis(30000),
            storage: StorageType::Memory,
            ..Default::default()
        })
        .await?;

    let kv_c2s = jetstream
        .create_key_value(async_nats::jetstream::kv::Config {
            bucket: "sessions_ly2eg".to_string(),
            description: "Maps Lightyear Client IDs to Edgegap Session IDs".to_string(),
            max_value_size: 1024,
            // shouldn't need long for the client to receive token, and make connection to gameserver.
            max_age: Duration::from_millis(30000),
            storage: StorageType::Memory,
            ..Default::default()
        })
        .await?;

    Ok((kv_s2c, kv_c2s))
}

async fn verify_deployment(state: &MatchmakerState) -> Result<(), async_nats::Error> {
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

    let deployment = deployment_status_get(config, settings.deployment_id.as_str())
        .await
        .unwrap_or_else(|e| {
            error!("Deployment ID not found: {}\n {e}", settings.deployment_id);
            std::process::exit(1);
        });

    if !deployment.running {
        error!("Deployment is not running, aborting.");
        std::process::exit(1);
    }

    let link = deployment
        .ports
        .expect("No ports in deployement")
        .iter()
        .next()
        .expect("No port in deployment")
        .1
        .link
        .clone()
        .expect("No link?");

    info!(
        "✅ {} @ {} :: {} start_time: {} link: {}",
        settings.app_name,
        settings.app_version,
        settings.deployment_id,
        deployment.start_time,
        link
    );

    Ok(())
}

// /// Start a detached task that listens for incoming TCP connections and sends `ConnectToken`s to clients
// fn start_netcode_authentication_task(
//     game_server_addr: SocketAddr,
//     auth_backend_addr: SocketAddr,
//     protocol_id: u64,
//     private_key: Key,
//     client_ids: Arc<RwLock<HashSet<u64>>>,
// ) {
//     IoTaskPool::get()
//         .spawn(Compat::new(async move {
//             info!(
//                 "Listening for ConnectToken requests on {}",
//                 auth_backend_addr
//             );
//             let listener = tokio::net::TcpListener::bind(auth_backend_addr)
//                 .await
//                 .unwrap();
//             loop {
//                 // received a new connection
//                 let (mut stream, _) = listener.accept().await.unwrap();

//                 // assign a new client_id
//                 let client_id = loop {
//                     let client_id = rand::random();
//                     if !client_ids.read().unwrap().contains(&client_id) {
//                         break client_id;
//                     }
//                 };

//                 let token =
//                     ConnectToken::build(game_server_addr, protocol_id, client_id, private_key)
//                         .generate()
//                         .expect("Failed to generate token");

//                 let serialized_token = token.try_into_bytes().expect("Failed to serialize token");
//                 trace!(
//                     "Sending token {:?} to client {}. Token len: {}",
//                     serialized_token,
//                     client_id,
//                     serialized_token.len()
//                 );
//                 stream
//                     .write_all(&serialized_token)
//                     .await
//                     .expect("Failed to send token to client");
//             }
//         }))
//         .detach();
// }

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
