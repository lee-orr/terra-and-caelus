use bevy::prelude::*;

use crate::{
    assets::GameAssets,
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
type TileDisplay<'a> = (&'a Children, Entity, &'a Backing, &'a Cell);

fn display_tiles(
    query: Query<TileDisplay, ChangedTile>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    for (child, entity, backing, cell) in query.iter() {
        let (soil, cell) = get_tile_image(backing, cell, assets.as_ref());
        commands.entity(entity).insert(soil);
        if let Some(child) = child.first() {
            if let Some(cell) = cell {
                commands.entity(*child).insert((cell, Visibility::Visible));
            } else {
                commands.entity(*child).insert(Visibility::Hidden);
            }
        }
    }
}

fn get_tile_image(
    backing: &Backing,
    cell: &Cell,
    assets: &GameAssets,
) -> (Handle<Image>, Option<Handle<Image>>) {
    (
        match backing {
            Backing::Water => assets.water.clone(),
            Backing::FertileSoil => assets.fertile_ground.clone(),
            Backing::HarshSoil => assets.harsh_ground.clone(),
            Backing::DepletedSoil => assets.depleted_ground.clone(),
        },
        match cell {
            Cell::Empty => None,
            Cell::Moss => Some(assets.moss.clone()),
            Cell::Flowers => Some(assets.flower.clone()),
        },
    )
    // match (backing, cell) {
    //     (Backing::Water, _) => Color::rgba(0., 0., 0., 0.),
    //     (Backing::FertileSoil, Cell::Empty) => Color::rgb(0.4, 0.8, 0.8),
    //     (Backing::FertileSoil, Cell::Moss) => Color::rgb(0.2, 0.6, 0.3),
    //     (Backing::HarshSoil, Cell::Empty) => Color::rgb(0.3, 0.2, 0.05),
    //     (Backing::HarshSoil, Cell::Moss) => Color::rgb(0.3, 0.4, 0.1),
    //     (Backing::DepletedSoil, Cell::Empty) => Color::rgb(0.2, 0.1, 0.05),
    //     (Backing::DepletedSoil, Cell::Moss) => Color::rgb(0.4, 0.5, 0.01),
    //     (Backing::FertileSoil, Cell::Flowers) => Color::rgb(0.7, 0.6, 0.1),
    //     (Backing::HarshSoil, Cell::Flowers) => Color::rgb(0.6, 0.5, 0.05),
    //     (Backing::DepletedSoil, Cell::Flowers) => Color::rgb(0.6, 0.3, 0.04),
    // }
}
