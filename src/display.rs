use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    states::AppState,
    tile::{Ground, Plant, PlantDefinitions, TileAsset, TILE_WORLD_SIZE},
};

pub struct TileDisplayPlugin;

impl Plugin for TileDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(display_tiles.in_set(OnUpdate(AppState::InGame)));
    }
}

type ChangedTile = AnyOf<(Changed<Ground>, Changed<Plant>)>;
type TileDisplay<'a> = (&'a Children, Entity, &'a Ground, &'a Plant);

fn display_tiles(
    query: Query<TileDisplay, ChangedTile>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    plants: Res<PlantDefinitions>,
) {
    for (child, entity, backing, cell) in query.iter() {
        let (soil, cell) = get_tile_image(backing, cell, assets.as_ref(), plants.as_ref());
        commands.entity(entity).insert(soil);
        if let Some(child) = child.first() {
            if let Some(cell) = cell {
                commands.entity(*child).insert((
                    cell.0,
                    Sprite {
                        color: cell.1,
                        custom_size: Some(TILE_WORLD_SIZE * Vec2::ONE),
                        ..Default::default()
                    },
                    Visibility::Visible,
                ));
            } else {
                commands.entity(*child).insert(Visibility::Hidden);
            }
        }
    }
}

fn get_tile_image(
    backing: &Ground,
    cell: &Plant,
    assets: &GameAssets,
    plants: &PlantDefinitions,
) -> (Handle<Image>, Option<(Handle<Image>, Color)>) {
    (
        match backing {
            Ground::Water => assets.water.clone(),
            Ground::Ground(6..) => assets.fertile_ground.clone(),
            Ground::Ground(1..=5) => assets.harsh_ground.clone(),
            Ground::Ground(_) => assets.depleted_ground.clone(),
        },
        match cell {
            Plant::Empty => None,
            Plant::Plant(p) => plants
                .assets
                .get(*p)
                .map(|TileAsset(asset, c)| (asset.clone(), *c)),
        },
    )
}
