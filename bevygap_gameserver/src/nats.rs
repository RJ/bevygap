use futures::StreamExt;
use std::{str::from_utf8, time::Duration};

// use serde::{Deserialize, Serialize};

use async_nats::{
    jetstream::{self, consumer::PushConsumer, stream::DiscardPolicy},
    Client, ConnectError,
};

pub async fn setup_nats() -> Result<Client, ConnectError> {
    // Use the NATS_URL env variable if defined, otherwise fallback to the default.
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());

    // Create an unauthenticated connection to NATS.
    async_nats::connect(nats_url).await
}

// we want to create a new jetstream kv bucket using our public IP in the key.
// within this bucket we will store sessions.
pub async fn setup_nats_bits(client: &Client) -> Result<(), async_nats::Error> {
    let inbox = client.new_inbox();
    let jetstream = jetstream::new(client.clone());
    let stream_name = String::from("GAMESERVERS");

    let public_ip = "0.0.0.0";

    let bucket = format!("gameserver");

    dbg!("Bucket= {}", bucket.clone());

    // Make a KV bucket for this gameserver, keyed by public IP.
    let kv = jetstream
        .create_key_value(async_nats::jetstream::kv::Config {
            bucket: bucket.to_string(),
            ..Default::default()
        })
        .await?;

    let consumer = jetstream
        .get_stream(kv.name.as_str())
        .await?
        .create_consumer(async_nats::jetstream::consumer::pull::Config {
            ..Default::default()
        })
        .await?;

    // dbg!("KV = {}", kv);

    let mut messages = consumer.messages().await?;
    let message = messages.next().await.unwrap()?;
    let metadata = message.info()?;
    println!(
        "{} @ {} -> {}",
        message.subject,
        metadata.stream_sequence,
        from_utf8(&message.payload)?
    );

    // // Create a stream and a consumer.
    // // We can chain the methods.
    // // First we create a stream and bind to it.
    // let consumer: PushConsumer = jetstream
    //     .create_stream(jetstream::stream::Config {
    //         name: stream_name,
    //         subjects: vec![topic.to_string()],
    //         ..Default::default()
    //     })
    //     .await?
    //     // Then, on that `Stream` use method to create Consumer and bind to it too.
    //     .create_consumer(jetstream::consumer::push::Config {
    //         deliver_subject: inbox.clone(),
    //         inactive_threshold: Duration::MAX,
    //         ..Default::default()
    //     })
    //     .await?;

    // // Attach to the messages iterator for the Consumer.
    // let mut messages = consumer.messages().await?.take(10);

    // // Iterate over messages.
    // while let Some(message) = messages.next().await {
    //     let message = message?;
    //     println!(
    //         "got message on subject {} with payload {:?}",
    //         message.subject,
    //         from_utf8(&message.payload)?
    //     );

    //     // acknowledge the message
    //     message.ack().await?;
    // }

    Ok(())
}
