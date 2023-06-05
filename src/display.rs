use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    states::AppState,
    tile::{Ground, Plant, PlantDefinitions, TileAsset, FIRE_DURATION, TILE_WORLD_SIZE},
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
            Ground::Empty => assets.void.clone(),
            Ground::Soil(false) => assets.soil.clone(),
            Ground::Soil(true) => assets.fertile_soil.clone(),
            Ground::Sand(false) => assets.sand.clone(),
            Ground::Sand(true) => assets.fertile_sand.clone(),
            Ground::Rock(false) => assets.rock.clone(),
            Ground::Rock(true) => assets.fertile_rock.clone(),
        },
        match cell {
            Plant::Empty => None,
            Plant::Plant(p) => plants
                .assets
                .get(p.as_str())
                .map(|TileAsset(asset, c)| (asset.clone(), *c)),
            Plant::Fire(remaining) => Some((
                assets.fire.clone(),
                Color::rgba(
                    1.,
                    1.,
                    1.,
                    ((*remaining as f32) / (FIRE_DURATION as f32)).clamp(0., 1.),
                ),
            )),
        },
    )
}
