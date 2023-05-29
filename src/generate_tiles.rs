use std::ops::Div;

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{
    states::AppState,
    tile::{Backing, Cell, Tile, TILE_WORLD_SIZE},
};

pub struct TileGeneratorPlugin;

impl Plugin for TileGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(NoisyGenerator(Vec2::new(15.130, 24124.563), 139.524))
            .add_system(generate_tiles.in_schedule(OnEnter(AppState::InGame)));
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

fn generate_tiles(mut commands: Commands, mut generator: ResMut<NoisyGenerator>) {
    for x in -25..25 {
        for y in -25..25 {
            let backing = generator
                .select_option(&[
                    Backing::Water,
                    Backing::FertileSoil,
                    Backing::FertileSoil,
                    Backing::FertileSoil,
                ])
                .cloned()
                .unwrap_or_default();
            let cell = generator
                .select_option(&[
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Moss,
                ])
                .cloned()
                .unwrap_or_default();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
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
            ));
        }
    }
}
