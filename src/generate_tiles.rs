use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    level_asset::{CurrentLevel, CurrentLevelHotReload, LevelAsset},
    states::AppState,
    tile::TILE_WORLD_SIZE,
};

pub struct TileGeneratorPlugin;

impl Plugin for TileGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(generate_tiles.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                generate_tiles
                    .in_base_set(CoreSet::PostUpdate)
                    .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::F1))),
            )
            .add_system(
                generate_tiles.in_base_set(CoreSet::PostUpdate).run_if(
                    in_state(AppState::InGame).and_then(on_event::<CurrentLevelHotReload>()),
                ),
            );
    }
}

#[derive(Component)]
pub struct Level;

fn generate_tiles(
    mut commands: Commands,
    existing_levels: Query<Entity, With<Level>>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    for entity in existing_levels.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let Some(current_level) = current_level.0.as_ref() else { return; };
    let Some(level) = level_assets.get(current_level) else { return;};

    commands
        .spawn((SpatialBundle::default(), Level))
        .with_children(|p| {
            for (tile, (ground, plant)) in level.tiles.0.iter() {
                p.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(
                            Vec3::new(tile.0 as f32, tile.1 as f32, 0.) * TILE_WORLD_SIZE,
                        ),
                        ..default()
                    },
                    *ground,
                    plant.clone(),
                    *tile,
                ))
                .with_children(|p| {
                    p.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                            ..default()
                        },
                        ..default()
                    });
                });
            }
        });
}
