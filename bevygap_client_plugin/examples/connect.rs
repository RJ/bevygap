use bevy::prelude::*;
use bevygap_client_plugin::prelude::*;
use lightyear::prelude::client::ClientConfig;

// This will crash, but only after successfully demonstrating the matchmaking part.
// (see the INFO level logs)

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(BevygapClientPlugin);
    app.insert_resource(ClientConfig::default());
    app.add_systems(Startup, |mut commands: Commands| {
        commands.bevygap_connect_client();
    });
    app.run();
}
