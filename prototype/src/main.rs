mod viewport;
mod level;
mod components;
mod utils;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use viewport::*;
use level::*;
use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tears of the Moon Palace".into(),
                resolution: (1366., 768.).into(),
                resizable: false,
                position: WindowPosition::Automatic,
                canvas: Some("#game".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ViewportPlugin)
        .add_plugin(LevelsPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
