/// Plugin that fetches the context from the Edgegap API, which contains
/// information relevant to the deployment of this gameserver, such as its
/// location, public IP, and other metadata.
use crate::arbitrium_env::ArbitriumEnv;
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;

pub struct EdgegapContextPlugin;

impl Plugin for EdgegapContextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fetch_context);
    }
}

#[derive(Resource, Debug, Clone)]
pub struct ArbitriumContext {
    context: serde_json::Map<String, serde_json::Value>,
}

impl ArbitriumContext {
    pub fn location(&self) -> String {
        let location = self
            .context
            .get("location")
            .expect("Missing location key in context");
        let city = location.get("city").expect("Missing city key in context");
        let country = location
            .get("country")
            .expect("Missing country key in context");
        format!("{}, {}", city, country)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.context).expect("Failed to serialize context to JSON")
    }

    pub fn sockets(&self) -> u32 {
        let serde_json::Value::Number(sockets) = self
            .context
            .get("sockets")
            .expect("Missing sockets key in context")
        else {
            panic!("Sockets is not a number!");
        };
        sockets.as_u64().expect("Sockets is not a number") as u32
    }

    pub fn top_level_string(&self, key: &str) -> String {
        self.context
            .get(key)
            .expect("Missing {str} key in context")
            .as_str()
            .expect("{str} is not a string")
            .to_string()
    }

    pub fn request_id(&self) -> String {
        self.top_level_string("request_id")
    }

    pub fn public_ip(&self) -> String {
        self.top_level_string("public_ip")
    }

    pub fn fqdn(&self) -> String {
        self.top_level_string("fqdn")
    }
}

/// Load context from the Edgegap API and insert into world resource.
async fn fetch_context_from_api(
    context_url: &str,
    context_token: &str,
) -> Result<ArbitriumContext, async_nats::Error> {
    let context_response = crate::http_client::get_context(context_url, context_token).await?;

    let serde_json::Value::Object(context_map) = context_response else {
        panic!("Context is not an object");
    };
    info!("Context fetched: {:?}", context_map);

    Ok(ArbitriumContext {
        context: context_map,
    })
}

pub fn fetch_context(runtime: ResMut<TokioTasksRuntime>, arb_env: Res<ArbitriumEnv>) {
    let context_url = arb_env.context_url.clone();
    let context_token = arb_env.context_token.clone();
    info!("Fetching context: {context_url} ::::  {context_token}");

    runtime.spawn_background_task(|mut ctx| async move {
        let arb_context = fetch_context_from_api(&context_url, &context_token)
            .await
            // .expect("Failed to fetch context from Edgegap API");
            .unwrap_or_else(|_err| {
                error!("Failed to fetch context from Edgegap API: {_err}");
                panic!("Failed to fetch context");
                // // panic, or use a fake value:
                // warn!("Using fake context!");
                // let mut context = serde_json::Map::new();
                // context.insert("fake_data".into(), "lol".into());
                // context.insert("fqdn".into(), "rj.example.com".into());
                // ArbitriumContext { context }
            });
        info!("Got Context: {arb_context:?}");
        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(arb_context);
        })
        .await;
    });
}
