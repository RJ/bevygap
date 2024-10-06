use bevy::prelude::*;
use bevygap_gameserver::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Close the window to return to the main function".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BevygapGameserverPlugin { mock_env: true })
        .add_systems(Startup, setup)
        .run();

    println!("Hello, world!");
}

fn setup() {
    info!("Hello from setup!");
}
