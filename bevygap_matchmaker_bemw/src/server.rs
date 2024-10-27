use bevy::tasks::TaskPool;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use bevy::{prelude::*, tasks::TaskPoolBuilder};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crate::messages::{self, C2SMessage, S2CMessage};

pub struct BevygapMatchmakerServerPlugin;

impl Plugin for BevygapMatchmakerServerPlugin {
    fn build(&self, mut app: &mut App) {
        // Before we can register the potential message types, we
        // need to add the plugin
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
        messages::server_register_network_messages(&mut app);

        app.add_systems(Startup, setup_networking);
        app.add_systems(Update, (handle_connection_events, handle_messages));
        app.add_systems(
            Update,
            reap_timedout_connections.run_if(on_timer(Duration::from_secs(10))),
        );

        // We have to insert the WS [`NetworkSettings`] with our chosen settings.
        app.insert_resource(NetworkSettings::default());
        app.init_resource::<MMConnections>();
    }
}

// map of ConnectionId to channels for exchanging async msgs with nats.
// todo.

// On the server side, you need to setup networking. You do not need to do so at startup, and can start listening
// at any time.
fn setup_networking(
    mut net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    let ip_address = "0.0.0.0".parse().expect("Could not parse ip address");

    info!("Address of the server: {}", ip_address);

    // SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
    let socket_address = SocketAddr::new(ip_address, 8080);

    match net.listen(socket_address, &task_pool.0, &settings) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    info!("Matchmaker websocket server listening on {socket_address}");
}

enum ClientState {
    Connected(Instant),
    FindingSession(Instant),
}

#[derive(Default, Resource)]
struct MMConnections {
    connections: HashMap<ConnectionId, ClientState>,
}

impl MMConnections {
    fn add(&mut self, conn_id: ConnectionId) {
        self.connections
            .insert(conn_id, ClientState::Connected(Instant::now()));
    }
    fn searching(&mut self, conn_id: ConnectionId) {
        self.connections
            .insert(conn_id, ClientState::FindingSession(Instant::now()));
    }
    fn remove(&mut self, conn_id: &ConnectionId) {
        self.connections.remove(conn_id);
    }
    fn remove_timed_out_connected(
        &mut self,
        handshake_dur: Duration,
        session_dur: Duration,
    ) -> Vec<ConnectionId> {
        self.connections
            .extract_if(|_, v| match v {
                ClientState::Connected(at) => (Instant::now() - *at) > handshake_dur,
                ClientState::FindingSession(at) => (Instant::now() - *at) > session_dur,
            })
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
    }
}

fn reap_timedout_connections(
    mut conns: ResMut<MMConnections>,
    net: Res<Network<WebSocketProvider>>,
) {
    let handshake_dur = Duration::from_secs(5);
    let session_dur = Duration::from_secs(300);
    let message = S2CMessage::Error(400, "time out".into());
    for conn_id in conns.remove_timed_out_connected(handshake_dur, session_dur) {
        warn!("Timeout conn_id {conn_id}");
        let _ = net.send_message(conn_id, message.clone());
        let _ = net.disconnect(conn_id);
    }
}

fn handle_connection_events(
    // mut commands: Commands,
    // net: Res<Network<WebSocketProvider>>,
    mut network_events: EventReader<NetworkEvent>,
    mut conns: ResMut<MMConnections>,
) {
    for event in network_events.read() {
        info!("{event:?}");
        match event {
            NetworkEvent::Connected(conn_id) => {
                // set a Timeout to disconnect if don't get the wannaplay msg in time.
                // would rather do this than automatically start a session request, to avoid
                // DoSing ourselves if a non game client connects to the websocket for whatever reason.
                conns.add(*conn_id);
            }
            NetworkEvent::Disconnected(conn_id) => {
                conns.remove(conn_id);
            }
            NetworkEvent::Error(e) => {
                error!("MM Network Error: {e:?}");
            }
        }
    }
}

// Receiving a new message is as simple as listening for events of `NetworkData<T>`
fn handle_messages(
    mut new_messages: ResMut<Events<NetworkData<messages::C2SMessage>>>,
    net: Res<Network<WebSocketProvider>>,
    mut conns: ResMut<MMConnections>,
) {
    for message in new_messages.drain() {
        let conn_id = message.source().clone();
        info!("RCVD {message:?}");
        match message.into_inner() {
            C2SMessage::WannaPlay { client_ip: _ } => {
                // TODO dispatch to NATS
                let message = S2CMessage::Acknowledged;
                match net.send_message(conn_id, message) {
                    Ok(()) => {
                        conns.searching(conn_id);
                    }
                    Err(e) => {
                        warn!("Error sending to client {e:?}");
                        let _ = net.disconnect(conn_id);
                        conns.remove(&conn_id);
                    }
                }
            }
        }
    }
}
