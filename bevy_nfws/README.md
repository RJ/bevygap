# bevy_nfws - No Frills WebSocket

No-frills websocket client, wasm + native.

Uses tokio-tungstenite-wasm for websocket communication in an async task, and some channels 
to bridge you into sync bevy-land.

Used to talk to a matchmaker in [bevygap_client_plugin](https://github.com/RJ/bevygap/).

### Example usage

```rust
use bevy::prelude::*;
use bevy_nfws::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(NfwsPlugin);
    app.add_systems(Startup, connect);
    app.add_systems(Update, poll);
    app.run();
}

/// Connect by spawning an entity with a NfwsHandle component.
/// bevy_nfws will spawn a background task to handle the websocket connection.
fn connect(mut commands: Commands) {
    let url = "ws://echo.websocket.org/".to_string();
    info!("Connecting to {url}");
    let mut handle = NfwsHandle::new(url);
    handle.send_text("Hello, World 1");
    handle.send_text("Hello, World 2");
    commands.spawn(handle);
}

/// Poll for events on the NfwsHandle.
fn poll(mut q: Query<(Entity, &mut NfwsHandle)>, mut commands: Commands) {
    // there's only one ws active, but you could have many:
    let (e, mut handle) = q.single_mut();
    // consume events until there are none, or the socket closes:
    loop {
        match handle.next_event() {
            NfwsPollResult::Event(ev) => {
                // New event from websocket
                info!("EV: {ev:?}");
                /*
                match ev {
                    NfwsEvent::Connecting => todo!(),
                    NfwsEvent::Connected => todo!(),
                    NfwsEvent::TextMessage(_String) => todo!(),
                    NfwsEvent::BinaryMessage(_Vec) => todo!(),
                    NfwsEvent::Error(nfws_err) => todo!(),
                    NfwsEvent::Closed(_) => todo!(),
                }
                */
                // Check again, could be more than one event ready this tick.
                continue;
            }
            NfwsPollResult::Empty => {
                // No new events.
                break;
            }
            NfwsPollResult::Closed => {
                info!("EV None = closed, despawning");
                commands.entity(e).despawn();
                break;
            }
        }
    }

}

```
