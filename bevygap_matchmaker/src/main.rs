use async_nats::Client;
use clap::Parser;
use edgegap::apis::applications_api::*;
use edgegap::apis::configuration::*;
use edgegap::apis::deployments_api::*;
use futures::stream::StreamExt;
use log::*;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::*, util::*};

mod session_service;

fn edgegap_configuration(settings: &Settings) -> Configuration {
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
}

impl MatchmakerState {
    pub(crate) fn nats_client(&self) -> Client {
        self.nats_client.clone()
    }
    pub(crate) fn configuration(&self) -> &Configuration {
        &self.api_config
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    info!("Starting Edgegap Matchmaker");
    let nats_client = connect_to_nats().await.unwrap();

    let settings = Settings::parse();

    let api_config = edgegap_configuration(&settings);

    let mm_state = MatchmakerState {
        nats_client: nats_client.clone(),
        api_config,
        settings,
    };

    let _deployments = load_deployments(&mm_state).await?;

    let state = mm_state.clone();
    let watcher = tokio::spawn(async move {
        match watch_for_gameserver_announcements(&state).await {
            Ok(_) => info!("Gameserver announcement watcher completed"),
            Err(e) => error!("Error in gameserver announcement watcher: {}", e),
        }
    });

    let state = mm_state.clone();
    let _session_service = tokio::spawn(async move {
        match session_service::session_request_supervisor(&state).await {
            Ok(_) => info!("Session service completed"),
            Err(e) => error!("Error in session service: {}", e),
        }
    });

    // just to block from exiting:
    watcher.await.unwrap();

    // shouldn't get here
    info!("Edgegap Matchmaker exiting");
    Ok(())
    // dbg!(deployments);
}

async fn load_deployments(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let config = state.configuration();
    let settings = &state.settings;

    let app = application_get(&config, settings.app_name.as_str())
        .await
        .unwrap_or_else(|e| panic!("Edgegap API doesn't know this application name: {e}"));

    info!(
        "🟢 Application '{}', active: {}, last_updated: {}",
        app.name, app.is_active, app.last_updated
    );

    let app_versions = app_versions_get(&config, settings.app_name.as_str())
        .await
        .unwrap_or_else(|e| panic!("Failed fetching versions: {e}"));

    let versions = app_versions.versions.expect("No versions found");

    let active_versions = versions
        .iter()
        .filter(|v| v.is_active == Some(true))
        .collect::<Vec<_>>();

    let mut is_selected_version_active = false;

    for version in active_versions {
        info!("🟢 Application version '{}' is active.", version.name);
        if version.name == settings.app_version {
            is_selected_version_active = true;
        }
    }

    if !is_selected_version_active {
        warn!(
            "🔴 Selected version {} is not active.",
            settings.app_version
        );
        std::process::exit(1);
    }

    let deployments = deployments_get(&config)
        .await
        .unwrap_or_else(|e| panic!("Failed fetching deployments: {e}"));

    let deployments = deployments.data.expect("No deployments found");

    if deployments.is_empty() {
        warn!("🔴 No deployments found");
        // std::process::exit(1);
    } else {
        for deployment in deployments {
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
            let icon = if deployment.ready { "🟢" } else { "🟠" };
            info!(
                "{} DEPLOYMENT {} start_time: {} ready: {} link: {}",
                icon, deployment.request_id, deployment.start_time, deployment.ready, link
            );
        }
    }
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
