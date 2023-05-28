use bevy::prelude::*;

use crate::{
    states::AppState,
    tile::{Backing, Cell},
};

pub struct TileDisplayPlugin;

impl Plugin for TileDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(display_tiles.in_set(OnUpdate(AppState::InGame)));
    }
}

type ChangedTile = AnyOf<(Changed<Backing>, Changed<Cell>)>;
type TileDisplay<'a> = (&'a mut Sprite, &'a Backing, &'a Cell);

fn display_tiles(mut query: Query<TileDisplay, ChangedTile>) {
    for (mut sprite, backing, cell) in query.iter_mut() {
        sprite.color = get_tile_color(backing, cell);
    }
}

fn get_tile_color(backing: &Backing, cell: &Cell) -> Color {
    match (backing, cell) {
        (Backing::Empty, Cell::Empty) => Color::rgba(0., 0., 0., 0.),
        (Backing::Empty, Cell::Moss) => Color::rgb(0.1, 0.4, 0.2),
        (Backing::FertileSoil, Cell::Empty) => Color::rgb(0.4, 0.3, 0.1),
        (Backing::FertileSoil, Cell::Moss) => Color::rgb(0.2, 0.6, 0.3),
        (Backing::HarshSoil, Cell::Empty) => Color::rgb(0.3, 0.2, 0.05),
        (Backing::HarshSoil, Cell::Moss) => Color::rgb(0.3, 0.4, 0.1),
        (Backing::DepletedSoil, Cell::Empty) => Color::rgb(0.2, 0.1, 0.05),
        (Backing::DepletedSoil, Cell::Moss) => Color::rgb(0.4, 0.5, 0.01),
    }
}
