use bevygap_shared::nats::*;
// use tracing_subscriber;
use tracing_subscriber::{layer::*, util::*};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // Start logging to console
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();

    let _nats = BevygapNats::new_and_connect("bevygap_nats_test")
        .await
        .unwrap();
    println!("NATS connected OK!");
}
