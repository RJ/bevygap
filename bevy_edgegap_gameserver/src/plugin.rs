// use crate::arbitrium_env::ArbitriumEnv;
// use crate::http_client::*;
use bevy::prelude::*;
// use bevy::tasks::block_on;
use futures::StreamExt;
use std::str::from_utf8;

use bevy_tokio_tasks::{TaskContext, TokioTasksPlugin, TokioTasksRuntime};

use crate::arbitrium_env::edgegap_environment_variables_reader_plugin;
use crate::edgegap_context_plugin::{ArbitriumContext, EdgegapContextPlugin};

/// Plugin for gameservers that run on edgegap.
pub struct EdgeGapGameServerPlugin;

impl Plugin for EdgeGapGameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(edgegap_environment_variables_reader_plugin);
        app.add_plugins(TokioTasksPlugin::default());
        app.add_plugins(EdgegapContextPlugin);

        app.add_systems(
            Update,
            (context_added, setup_nats)
                .chain()
                .run_if(resource_added::<ArbitriumContext>),
        );

        app.add_systems(Update, nats_added.run_if(resource_added::<Nats>));
    }
}

fn context_added(context: Res<ArbitriumContext>) {
    info!("CONTEXT added: {context:?}");
    info!("CONTEXT fqdn: {}", context.fqdn());
    // info!("CONTEXT request_id: {}", context.request_id());
    // info!("CONTEXT location: {}", context.location());
    // info!("CONTEXT sockets: {}", context.sockets());
    // info!("CONTEXT public_ip: {}", context.public_ip());
}

fn nats_added(nats: Res<Nats>, runtime: ResMut<TokioTasksRuntime>) {
    info!("NATS added.");
    // let kv_consumer = nats.kv_consumer.clone();
    // runtime.spawn_background_task(|mut ctx| async move {
    //     // Now we want to run polling to marshall events from tokio/nats <--> bevy events.
    //     info!("Starting KV consumer loop");
    //     loop {
    //         match read_kv_consumer(&kv_consumer).await {
    //             Ok(_) => (),
    //             Err(e) => error!("Error reading KV consumer: {}", e),
    //         }
    //     }
    // });
}

fn setup_nats(runtime: ResMut<TokioTasksRuntime>, arb_context: Res<ArbitriumContext>) {
    let arb_context_bytes = arb_context.to_bytes();
    let fqdn = arb_context.fqdn();
    runtime.spawn_background_task(|mut ctx| async move {
        let nats_key = fqdn.replace('.', "_");
        let nats = match connect_to_nats(nats_key).await {
            Ok(nats) => nats,
            Err(e) => {
                error!("Failed to setup NATS: {}", e);
                panic!("Failed to setup NATS");
            }
        };
        // Write our context to nats to announce our presence.
        nats.client
            .publish("gameserver.contexts", arb_context_bytes.into())
            .await
            .expect("Failed to write context to NATS");

        nats.client.flush().await.expect("Failed to flush NATS");

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(nats);
        })
        .await;
    });
}

async fn read_kv_consumer(
    kv_consumer: &async_nats::jetstream::consumer::Consumer<
        async_nats::jetstream::consumer::pull::Config,
    >,
) -> Result<(), async_nats::Error> {
    let mut messages = kv_consumer.messages().await?;
    let message = messages.next().await.unwrap()?;
    let metadata = message.info()?;
    info!(
        "KV_CONSUMER SAYS: {} @ {} -> {}",
        message.subject,
        metadata.stream_sequence,
        from_utf8(&message.payload)?
    );
    Ok(())
}

#[derive(Resource)]
pub struct Nats {
    pub client: async_nats::Client,
    // pub bucket: String,
    /// A unique key for this server, which is the FQDN with _ instead of .
    pub server_key: String,
    // pub inbox: String,
    // pub jetstream: async_nats::jetstream::Context,
    // pub kv: async_nats::jetstream::kv::Store,
    // pub kv_consumer:
    // async_nats::jetstream::consumer::Consumer<async_nats::jetstream::consumer::pull::Config>,
}

// impl Nats {
//     pub fn backchannel_subject(&self) -> String {
//         format!("gameserver.{}", self.server_key)
//     }
// }

/// Connect to NATS and create a KV bucket for this gameserver.
/// Write our context into the KV bucket.
/// This is all happening during startup before we have connected players, so it's ok to block.
async fn connect_to_nats(nats_key: String) -> Result<Nats, async_nats::Error> {
    info!("Setting up NATS");
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    info!("NATS_URL: {}", nats_url);
    // Connect to NATS.
    let client = async_nats::connect(nats_url).await?;

    // // no . in bucket name
    // let bucket = format!("gameserver:something"); //gameserver.{nats_key}");
    // info!("NATS KV BUCKET: {}", bucket);
    // // let inbox = client.new_inbox();
    // let jetstream = async_nats::jetstream::new(client.clone());
    // // let stream_name = String::from("GAMESERVERS");

    // // Make a KV bucket for this gameserver, keyed by public IP.
    // let kv = jetstream
    //     .create_key_value(async_nats::jetstream::kv::Config {
    //         bucket: bucket.clone(),
    //         ..Default::default()
    //     })
    //     .await?;

    // info!("Bucket created: {}", kv.name);

    // let kv_consumer = jetstream
    //     .get_stream(format!("KV_{}", kv.name.as_str()))
    //     .await?
    //     .create_consumer(async_nats::jetstream::consumer::pull::Config {
    //         ..Default::default()
    //     })
    //     .await?;

    Ok(Nats {
        client,
        // bucket,
        server_key: nats_key,
        // kv_consumer,
        // kv,
        // jetstream,
    })
}
