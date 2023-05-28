use bevy::prelude::*;

use crate::{tile::{Cell, Backing}, states::AppState};

pub struct TileDisplayPlugin;

impl Plugin for TileDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(display_tiles.in_set(OnUpdate(AppState::InGame)));
    }
}

fn display_tiles(mut query: Query<(&mut Sprite, &Backing, &Cell), AnyOf<(Changed<Backing>, Changed<Cell>)>>) {
    for (mut sprite, backing, cell) in query.iter_mut() {
        sprite.color = get_tile_color(backing, cell);
    }
}

fn get_tile_color(backing: &Backing, cell: &Cell) -> Color {
    match (backing, cell) {
        (Backing::Empty, Cell::Empty) => Color::rgba(0., 0., 0., 0.),
        (Backing::Empty, Cell::Moss) => Color::rgb(0.1, 0.4, 0.2),
        (Backing::Soil, Cell::Empty) => Color::rgb(0.4, 0.3, 0.1),
        (Backing::Soil, Cell::Moss) => Color::rgb(0.2, 0.6, 0.3),
    }
}