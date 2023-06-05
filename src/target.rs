use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    assets::GameAssets,
    control::{GainPower, Player, Power},
    states::AppState,
    tile::{Ground, Plant, PlantDefinitions, Tile, TileAsset, TILE_WORLD_SIZE},
};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_target.in_set(OnUpdate(AppState::InGame)))
            .add_system(process_target.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Reward {
    CompleteLevel,
    Fertilize,
    Burn,
    Seed,
    Drain,
}

#[derive(Component, Debug, Clone)]
pub struct Target(pub Tile, pub String, pub Reward);

#[derive(Component)]
pub struct UsedTarget;

fn setup_target(
    mut commands: Commands,
    targets: Query<(Entity, &Target), Without<Sprite>>,
    assets: Res<GameAssets>,
    plants: Res<PlantDefinitions>,
) {
    for (e, target) in targets.iter() {
        commands
            .entity(e)
            .insert(SpriteBundle {
                transform: Transform::from_translation(
                    Vec3::new(target.0 .0 as f32, target.0 .1 as f32, 1.) * TILE_WORLD_SIZE,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                    ..default()
                },
                texture: match target.2 {
                    Reward::CompleteLevel => &assets.goal,
                    Reward::Fertilize => &assets.shrine_fertilize,
                    Reward::Burn => &assets.shrine_fire,
                    Reward::Seed => &assets.shrine_seed,
                    Reward::Drain => &assets.shrine_drain,
                }
                .clone(),
                ..default()
            })
            .with_children(|p| {
                let Some((image, color)) = plants
                    .assets
                    .get(target.1.as_str())
                    .map(|TileAsset(asset, c)| (asset.clone(), *c)) else { return; };
                p.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(TILE_WORLD_SIZE * Vec2::ONE * 0.3),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    texture: image,
                    ..default()
                });
            });
    }
}

fn process_target(
    players: Query<&Player>,
    targets: Query<(Entity, &Target), Without<UsedTarget>>,
    tiles: Query<(&Tile, &Ground, &Plant)>,
    mut commands: Commands,
    mut gain_power: EventWriter<GainPower>,
) {
    let Ok(player) = players.get_single() else { return; };
    for (tile, _, plant) in tiles.iter() {
        if tile.0 == player.0 && tile.1 == player.1 {
            for (e, target) in targets.iter() {
                if target.0 == *tile {
                    let Plant::Plant(p) = plant else { continue; };
                    if p.as_str() == target.1 {
                        commands.entity(e).insert(UsedTarget).despawn_descendants();
                        match target.2 {
                            Reward::CompleteLevel => {
                                commands.insert_resource(NextState(Some(AppState::LevelComplete)));
                            }
                            Reward::Fertilize => gain_power.send(GainPower(Power::Fertilize)),
                            Reward::Burn => gain_power.send(GainPower(Power::Fire)),
                            Reward::Seed => gain_power.send(GainPower(Power::Seed)),
                            Reward::Drain => gain_power.send(GainPower(Power::Drain)),
                        };
                    }
                }
            }
        }
    }
}
