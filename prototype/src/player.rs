use bevy::prelude::*;

pub struct PlayerPlugin;
use systems::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

mod systems {
    use bevy::prelude::*;

    use crate::{components::*, utils::sprite};

    pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player = commands
            .spawn((
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Player {},
                Name::new("Player"),
            ))
            .id();

        let character = commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..default()
                    },
                    texture: asset_server.load(sprite("draft-150px.png")),
                    ..default()
                },
                Character {},
                Name::new("Chracter"),
            ))
            .id();

        commands.entity(player).add_child(character);
    }
}
