use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Backing {
    #[default]
    Empty,
    FertileSoil,
    HarshSoil,
    DepletedSoil,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Cell {
    #[default]
    Empty,
    Moss,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Tile(pub i8, pub i8);

impl From<Vec2> for Tile {
    fn from(value: Vec2) -> Self {
        Tile(value.x.floor() as i8, value.y.floor() as i8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fertalize(pub Tile);

pub const TILE_WORLD_SIZE: f32 = 20.;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Fertalize>();
    }
}