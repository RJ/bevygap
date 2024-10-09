use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use bevygap_shared::*;
use log::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::*, util::*};

struct AppState {
    bgnats: BevygapNats,
}

#[tokio::main]
async fn main() {
    setup_logging();

    let bgnats = BevygapNats::new_and_connect("bevygap_webhook_sink")
        .await
        .unwrap();
    let app_state = Arc::new(AppState { bgnats });

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/hook/:hookname", post(hook_handler))
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    info!(
        "bevygap_webhook_sink listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Webhook catcher</h1><p>Nothing to see here, move along.</p>")
}

async fn hook_handler(
    Path(hook_name): Path<String>,
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let subject = format!("webhook.{hook_name}");
    info!("NATS publish {subject} = {body}");
    state.bgnats.client().publish(subject, body.into()).await?;
    Ok("OK")
}

/// Serde deserialization decorator to map empty Strings to None,

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

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
