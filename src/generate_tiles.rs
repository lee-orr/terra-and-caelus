use std::ops::Div;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{
    states::AppState,
    tile::{Ground, Plant, PlantDefinitions, Tile, TILE_WORLD_SIZE},
};

pub struct TileGeneratorPlugin;

impl Plugin for TileGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(NoisyGenerator(Vec2::new(15.130, 24124.563), 139.524))
            .add_system(generate_tiles.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                generate_tiles
                    .in_base_set(CoreSet::PostUpdate)
                    .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::F1))),
            );
    }
}

#[derive(Resource)]
pub struct NoisyGenerator(Vec2, f32);

impl NoisyGenerator {
    pub fn generate_value(&mut self) -> f32 {
        let x = simplex_noise_2d_seeded(self.0, self.1);
        self.0 = Vec2::new(
            self.0.y * x.div(self.1) + self.0.x,
            self.1 * self.1 - self.0.x.div(self.0.y),
        );
        x
    }

    pub fn select_option<'a, T>(&mut self, options: &'a [T]) -> Option<&'a T> {
        if options.is_empty() {
            return None;
        }
        let value = self.generate_value();
        let index = (options.len() as f32 * value).abs().floor() as usize;
        options.get(index)
    }
}

#[derive(Component)]
pub struct Level;

fn generate_tiles(
    mut commands: Commands,
    existing_levels: Query<Entity, With<Level>>,
    mut generator: ResMut<NoisyGenerator>,
    plants: Res<PlantDefinitions>,
) {
    for entity in existing_levels.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((SpatialBundle::default(), Level))
        .with_children(|p| {
            let moss_id = plants.name_to_id.get("moss").expect("Moss isn't loaded");
            let other_flower = plants
                .name_to_id
                .get("other_flower")
                .expect("Moss isn't loaded");
            for x in -10..10 {
                for y in -10..10 {
                    let backing = generator
                        .select_option(&[
                            Ground::Water,
                            Ground::Ground(8),
                            Ground::Ground(5),
                            Ground::Ground(5),
                            Ground::Ground(5),
                            Ground::Ground(5),
                            Ground::Ground(5),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                            Ground::Ground(0),
                        ])
                        .cloned()
                        .unwrap_or_default();
                    let cell = generator
                        .select_option(&[
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Empty,
                            Plant::Plant(*moss_id),
                            Plant::Plant(*moss_id),
                            Plant::Plant(*moss_id),
                            Plant::Plant(*other_flower),
                        ])
                        .cloned()
                        .unwrap_or_default();
                    p.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(x as f32, y as f32, 0.) * TILE_WORLD_SIZE,
                            ),
                            ..default()
                        },
                        backing,
                        cell,
                        Tile(x, y),
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
            }
        });
}
