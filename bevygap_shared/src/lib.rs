use async_nats::jetstream::stream::Stream;
use async_nats::jetstream::{self, stream};
use async_nats::Client;
use std::time::Duration;

use log::*;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct BevygapNats {
    client: Client,
    kv_s2c: jetstream::kv::Store,
    kv_c2s: jetstream::kv::Store,
    kv_cert_digests: jetstream::kv::Store,
    kv_active_connections: jetstream::kv::Store,
    kv_unclaimed_sessions: jetstream::kv::Store,
    delete_session_stream: Stream,
}

const DELETE_SESSION_STREAM: &str = "edgegap_delete_session_q";

impl BevygapNats {
    /// Connects to NATS based on environment variables.
    pub async fn new_and_connect(nats_client_name: &str) -> Result<Self, async_nats::Error> {
        let client = Self::connect_to_nats(nats_client_name).await?;
        let (kv_s2c, kv_c2s) = Self::create_kv_buckets_for_session_mappings(client.clone()).await?;
        let kv_active_connections = Self::create_kv_active_connections(client.clone()).await?;
        let kv_cert_digests = Self::create_kv_cert_digests(client.clone()).await?;
        let kv_unclaimed_sessions = Self::create_kv_unclaimed_sessions(client.clone()).await?;
        let delete_session_stream = Self::create_session_delete_queue(&client).await?;
        Ok(Self {
            client,
            kv_s2c,
            kv_c2s,
            kv_cert_digests,
            kv_active_connections,
            kv_unclaimed_sessions,
            delete_session_stream,
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
    pub fn kv_active_connections(&self) -> &jetstream::kv::Store {
        &self.kv_active_connections
    }
    pub fn kv_unclaimed_sessions(&self) -> &jetstream::kv::Store {
        &self.kv_unclaimed_sessions
    }
    pub fn kv_cert_digests(&self) -> &jetstream::kv::Store {
        &self.kv_cert_digests
    }
    pub fn delete_session_stream(&self) -> &Stream {
        &self.delete_session_stream
    }

    /// Enqueues a job to delete a session id via the edgegap API
    pub async fn enqueue_session_delete(
        &self,
        session_id: String,
    ) -> Result<(), async_nats::Error> {
        let js = jetstream::new(self.client.clone());
        js.publish(
            format!("{DELETE_SESSION_STREAM}.{session_id}"),
            session_id.into(),
        )
        .await?
        .await?;
        Ok(())
    }

    async fn connect_to_nats(nats_client_name: &str) -> Result<Client, async_nats::Error> {
        info!("Setting up NATS, client name: {}", nats_client_name);
        let nats_host = std::env::var("NATS_HOST").expect("missing NATS_HOST env");
        let nats_user = std::env::var("NATS_USER").expect("Missing NATS_USER env");
        let nats_pass = std::env::var("NATS_PASS").expect("Missing NATS_PASS env");
        let insecure = std::env::var("NATS_INSECURE")
            .map(|_| true)
            .unwrap_or(false);
        // let nats_ca = std::env::var("NATS_CA").unwrap_or("./config/rootCA.pem".to_string());
        // let nats_cert =
        // std::env::var("NATS_CERT").unwrap_or("./config/client-cert.pem".to_string());
        // let nats_key = std::env::var("NATS_KEY").unwrap_or("./config/client-key.pem".to_string());

        info!("NATS_HOST: {}", nats_host);
        info!("NATS_USER: {}", nats_user);
        if insecure {
            info!("NATS_INSECURE: {}", insecure);
        }
        // info!("NATS_CA: {}", nats_ca);
        // info!("NATS_CERT: {}", nats_cert);
        // info!("NATS_KEY: {}", nats_key);

        let client = async_nats::ConnectOptions::new()
            .name(nats_client_name)
            .user_and_password(nats_user, nats_pass)
            .max_reconnects(10)
            .require_tls(!insecure)
            // .add_root_certificates(nats_ca.into())
            // .add_client_certificate(nats_cert.into(), nats_key.into())
            .connect(nats_host)
            .await?;

        info!("🟢 NATS connected");

        Ok(client)
    }

    pub async fn create_kv_active_connections(
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

    pub async fn create_kv_unclaimed_sessions(
        client: Client,
    ) -> Result<jetstream::kv::Store, async_nats::Error> {
        let jetstream = jetstream::new(client);
        let kv = jetstream
            .create_key_value(async_nats::jetstream::kv::Config {
                bucket: "unclaimed_sessions".to_string(),
                max_value_size: 1024,
                description: "Any session ids we get from the API are stored here, and if they key age gets too big, we delete the session via the API.".to_string(),
                ..Default::default()
            })
            .await?;
        Ok(kv)
    }

    pub async fn create_session_delete_queue(client: &Client) -> Result<Stream, async_nats::Error> {
        let js = jetstream::new(client.clone());
        let stream = js
            .create_stream(jetstream::stream::Config {
                name: "DELETE_SESSION_STREAM".to_string(),
                retention: stream::RetentionPolicy::WorkQueue,
                subjects: vec![format!("{DELETE_SESSION_STREAM}.*").to_string()],
                ..Default::default()
            })
            .await?;
        Ok(stream)
    }

    pub async fn create_kv_cert_digests(
        client: Client,
    ) -> Result<jetstream::kv::Store, async_nats::Error> {
        let jetstream = jetstream::new(client);
        let kv = jetstream
            .create_key_value(async_nats::jetstream::kv::Config {
                bucket: "cert_digests".to_string(),
                description: "Maps server public ip to their self-signed cert digests".to_string(),
                max_age: Duration::from_secs(86400 * 14),
                max_value_size: 1024,

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
                // storage: StorageType::File,
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
                // storage: StorageType::File,
                ..Default::default()
            })
            .await?;

        Ok((kv_s2c, kv_c2s))
    }
}
