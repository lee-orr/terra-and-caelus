use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Backing {
    #[default]
    Empty,
    Soil
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Moss,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tile(pub i8, pub i8);