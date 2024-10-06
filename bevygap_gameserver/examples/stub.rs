use bevy::prelude::*;
use bevygap_gameserver::plugin::EdgeGapGameServerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Close the window to return to the main function".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EdgeGapGameServerPlugin)
        .add_systems(Startup, setup)
        .run();

    println!("Hello, world!");
}

fn setup() {
    info!("Hello from setup!");
}
