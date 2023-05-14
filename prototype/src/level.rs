use bevy::prelude::*;

pub struct LevelsPlugin;
use systems::*;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
    }
}

mod systems {
    use bevy::prelude::*;

    use crate::{components::*, utils::sprite};

    pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
        let level = commands
            .spawn((SpatialBundle::default(), Level {}, Name::new("Level")))
        .id();

        let grid = commands
        .spawn((SpatialBundle::default(), Grid {}, Name::new("Grid")))
        .id();

        commands.entity(level).add_child(grid);

        for x in -50..50 {
            for y in -50..50 {
                let viewport_position = world_to_viewport(Vec3 {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                });
                let grid_tile = commands
                    .spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: viewport_position,
                                ..default()
                            },
                            texture: asset_server.load(sprite("xyz-112x64.png")),
                            ..default()
                        },
                        GridTile {},
                        Name::new(format!("Grid Tile ({}, {})", x, y)),
                    )).id();
                commands.entity(grid).add_child(grid_tile);
            }
        }
    }

    pub fn world_to_viewport(p: Vec3) -> Vec3 {
        Vec3 {
            x: p.x * 112.0 - 56.0,
            y: p.y * 64.0 - 32.0,
            z: p.z,
        }
    }
}
