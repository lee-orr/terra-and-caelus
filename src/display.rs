use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    states::AppState,
    tile::{Ground, Plants},
};

pub struct TileDisplayPlugin;

impl Plugin for TileDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(display_tiles.in_set(OnUpdate(AppState::InGame)));
    }
}

type ChangedTile = AnyOf<(Changed<Ground>, Changed<Plants>)>;
type TileDisplay<'a> = (&'a Children, Entity, &'a Ground, &'a Plants);

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
    backing: &Ground,
    cell: &Plants,
    assets: &GameAssets,
) -> (Handle<Image>, Option<Handle<Image>>) {
    (
        match backing {
            Ground::Water => assets.water.clone(),
            Ground::Ground(6..) => assets.fertile_ground.clone(),
            Ground::Ground(1..=5) => assets.harsh_ground.clone(),
            Ground::Ground(0) => assets.depleted_ground.clone(),
        },
        match cell {
            Plants::Empty => None,
            Plants::Moss => Some(assets.moss.clone()),
            Plants::Flowers => Some(assets.flower.clone()),
        },
    )
}
