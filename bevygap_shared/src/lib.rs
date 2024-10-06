use async_nats::jetstream;
use async_nats::jetstream::stream::StorageType;
use async_nats::Client;
use std::time::Duration;

use log::*;

#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct BevygapNats {
    client: Client,
    kv_s2c: jetstream::kv::Store,
    kv_c2s: jetstream::kv::Store,
    kv_sessions: jetstream::kv::Store,
}

impl BevygapNats {
    /// Connects to NATS based on environment variables.
    pub async fn new_and_connect(nats_client_name: &str) -> Result<Self, async_nats::Error> {
        let client = Self::connect_to_nats(nats_client_name).await?;
        let (kv_s2c, kv_c2s) = Self::create_kv_buckets_for_session_mappings(client.clone()).await?;
        let kv_sessions = Self::create_kv_sessions(client.clone()).await?;
        Ok(Self {
            client,
            kv_s2c,
            kv_c2s,
            kv_sessions,
        })
    }

    pub fn client(&self) -> Client {
        self.client.clone()
    }
    pub fn kv_s2c(&self) -> &jetstream::kv::Store {
        &self.kv_s2c
    }
    pub fn kv_c2s(&self) -> &jetstream::kv::Store {
        &self.kv_c2s
    }
    pub fn kv_sessions(&self) -> &jetstream::kv::Store {
        &self.kv_sessions
    }

    async fn connect_to_nats(nats_client_name: &str) -> Result<Client, async_nats::Error> {
        info!("Setting up NATS");
        let nats_host = std::env::var("NATS_HOST").unwrap_or("localhost:4222".to_string());
        let nats_ca = std::env::var("NATS_CA").unwrap_or("./config/rootCA.pem".to_string());
        let nats_cert =
            std::env::var("NATS_CERT").unwrap_or("./config/client-cert.pem".to_string());
        let nats_key = std::env::var("NATS_KEY").unwrap_or("./config/client-key.pem".to_string());

        info!("NATS_HOST: {}", nats_host);
        info!("NATS_CA: {}", nats_ca);
        info!("NATS_CERT: {}", nats_cert);
        info!("NATS_KEY: {}", nats_key);

        let client = async_nats::ConnectOptions::new()
            .name(nats_client_name)
            .max_reconnects(10)
            .require_tls(true)
            .add_root_certificates(nats_ca.into())
            .add_client_certificate(nats_cert.into(), nats_key.into())
            .connect(nats_host)
            .await?;

        info!("🟢 NATS connected");

        Ok(client)
    }

    pub async fn create_kv_sessions(
        client: Client,
    ) -> Result<jetstream::kv::Store, async_nats::Error> {
        let jetstream = jetstream::new(client);
        let kv = jetstream
            .create_key_value(async_nats::jetstream::kv::Config {
                bucket: "active_connections".to_string(),
                ..Default::default()
            })
            .await?;
        Ok(kv)
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
}
