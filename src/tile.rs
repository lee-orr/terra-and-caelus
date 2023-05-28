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
