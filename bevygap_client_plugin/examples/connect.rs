use bevy::prelude::*;
use bevygap_client_plugin::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(BevygapClientPlugin);
    app.add_systems(Startup, |mut commands: Commands| {
        commands.bevygap_connect_client();
    });
    app.run();
}
