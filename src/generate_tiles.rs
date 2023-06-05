use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    control::Player,
    level_asset::{CurrentLevel, CurrentLevelHotReload, LevelAsset},
    states::AppState,
    target::Target,
    tile::{GameEntity, TILE_WORLD_SIZE},
};

pub struct TileGeneratorPlugin;

impl Plugin for TileGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LevelLoaded>()
            .add_system(generate_tiles.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                generate_tiles
                    .in_base_set(CoreSet::PostUpdate)
                    .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::F1))),
            )
            .add_system(
                generate_tiles.in_base_set(CoreSet::PostUpdate).run_if(
                    in_state(AppState::InGame).and_then(on_event::<CurrentLevelHotReload>()),
                ),
            )
            .add_system(clear_level.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Level;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LevelLoaded;

fn generate_tiles(
    mut commands: Commands,
    existing_levels: Query<Entity, With<Level>>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<LevelAsset>>,
    mut loaded: EventWriter<LevelLoaded>,
    audio: Res<Audio>,
) {
    for entity in existing_levels.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let Some(current_level) = current_level.0.as_ref() else { return; };
    let Some(level) = level_assets.get(current_level) else { return;};

    commands
        .spawn((SpatialBundle::default(), Level))
        .with_children(|p| {
            for (tile, (ground, plant, game_entities)) in level.tiles.0.iter() {
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

                for ge in game_entities.iter() {
                    match ge {
                        GameEntity::Player => p.spawn((Player(tile.0, tile.1),)),
                        GameEntity::Target(t, r) => p.spawn(Target(*tile, t.clone(), *r)),
                    };
                }
            }
        });
    loaded.send(LevelLoaded);
}

fn clear_level(mut commands: Commands, existing_levels: Query<Entity, With<Level>>) {
    for entity in existing_levels.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.insert_resource(CurrentLevel(None));
}
