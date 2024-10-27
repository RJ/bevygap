use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};

use crate::messages::{self, C2SMessage};

/// Triggers a connection to the matchmaker and requests a session
#[derive(Event, Debug, Clone)]
pub struct MatchmakerRequest {
    mm_url: url::Url,
}

impl MatchmakerRequest {
    /// eg: "ws://127.0.0.1:8081/mm/url"
    pub fn from_url(url: &str) -> Self {
        let mm_url = url::Url::parse(url).expect("Unparsable mm URL");
        Self { mm_url }
    }
    pub fn url(&self) -> url::Url {
        self.mm_url.clone()
    }
}

/// Cancels any matchmaker connection and aborts pending requests
#[derive(Event, Debug, Clone, Default)]
pub struct CancelMatchmakerRequest;

#[derive(Clone, States, Default, Debug, Hash, PartialEq, Eq)]
pub enum MatchmakerConnectionState {
    #[default]
    Disconnected,
    Disconnecting,
    Connecting,
    Connected,
    // Error(String), // check in_state run condition includes this
}

pub struct BevygapMatchmakerClientPlugin;

impl Plugin for BevygapMatchmakerClientPlugin {
    fn build(&self, mut app: &mut App) {
        // You need to add the `EventworkPlugin` first before you can register
        // `ClientMessage`s
        app.add_plugins(bevy_eventwork::EventworkPlugin::<
            WebSocketProvider,
            bevy::tasks::TaskPool,
        >::default());

        // Make sure you insert the EventworkRuntime resource with your chosen Runtime
        app.insert_resource(EventworkRuntime(
            TaskPoolBuilder::new().num_threads(2).build(),
        ));

        // A good way to ensure that you are not forgetting to register
        // any messages is to register them where they are defined!
        messages::client_register_network_messages(&mut app);

        // We have to insert the WS [`NetworkSettings`] with our chosen settings.
        app.insert_resource(NetworkSettings::default());

        app.init_state::<MatchmakerConnectionState>();

        app.observe(observe_cancel);
        app.observe(observe_connect);

        app.add_systems(
            Update,
            (
                handle_incoming_messages
                    .run_if(on_event::<NetworkData<messages::S2CMessage>>())
                    .run_if(not(in_state(MatchmakerConnectionState::Disconnected))),
                handle_network_events.run_if(on_event::<NetworkEvent>()),
            ),
        );
    }
}

fn handle_incoming_messages(
    mut new_messages: ResMut<Events<NetworkData<messages::S2CMessage>>>,
    mut commands: Commands,
) {
    for new_message in new_messages.drain() {
        info!("RECV {new_message:?}");
        commands.trigger(new_message.into_inner());
        // we should probably maintain a higher level matchmaker state?
    }
}

fn handle_network_events(
    mut new_network_events: EventReader<NetworkEvent>,
    mut next_state: ResMut<NextState<MatchmakerConnectionState>>,
    net: Res<Network<WebSocketProvider>>,
) {
    for event in new_network_events.read() {
        info!("Received event {event:?}");
        // TODO thread client_ip in here via a resource?
        let message = C2SMessage::WannaPlay { client_ip: None };

        match event {
            NetworkEvent::Connected(_) => match net.send_message(ConnectionId { id: 0 }, message) {
                Ok(()) => {
                    next_state.set(MatchmakerConnectionState::Connected);
                }
                Err(e) => {
                    warn!("MM request err: {e:?}");
                    let _ = net.disconnect(ConnectionId { id: 0 });
                }
            },
            NetworkEvent::Disconnected(_) => {
                next_state.set(MatchmakerConnectionState::Disconnected);
            }
            NetworkEvent::Error(err) => {
                warn!("Disconnected, err: {err:?}");
                //  TODO error state? like if MM is down.
                next_state.set(MatchmakerConnectionState::Disconnected);
            }
        }
    }
}
fn observe_connect(
    trigger: Trigger<MatchmakerRequest>,
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
    mut next_state: ResMut<NextState<MatchmakerConnectionState>>,
) {
    if net.has_connections() {
        warn!("Already connected");
        return;
    }
    net.connect(trigger.event().url(), &task_pool.0, &settings);
    next_state.set(MatchmakerConnectionState::Connecting);
}

fn observe_cancel(
    _trigger: Trigger<CancelMatchmakerRequest>,
    net: ResMut<Network<WebSocketProvider>>,
    mut next_state: ResMut<NextState<MatchmakerConnectionState>>,
) {
    info!("observe_cancel");

    if !net.has_connections() {
        warn!("Not connected");
        return;
    }

    match net.disconnect(ConnectionId { id: 0 }) {
        Ok(()) => {
            next_state.set(MatchmakerConnectionState::Disconnecting);
        }
        Err(e) => {
            warn!("Err disconnecting from matchmaker: {e:?}");
        }
    }
}
