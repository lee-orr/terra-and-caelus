use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    states::AppState,
    tile::{Fertalize, Ground, Plant, PlantDefinition, PlantDefinitions, PlantFlower, Tile},
};

pub struct UpdateTilesPlugin;

impl Plugin for UpdateTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_tiles.run_if(
                in_state(AppState::InGame).and_then(on_timer(Duration::from_secs_f32(0.5))),
            ),
        )
        .add_system(fertilize_tiles.in_set(OnUpdate(AppState::InGame)))
        .add_system(plant_flowers_in_tiles.in_set(OnUpdate(AppState::InGame)));
    }
}

fn fertilize_tiles(
    query: Query<(Entity, &Tile, &Ground)>,
    mut fertilize: EventReader<Fertalize>,
    mut commands: Commands,
) {
    for fertilize in fertilize.iter() {
        let tile = &fertilize.0;
        if let Some((entity, _, backing)) = query.iter().find(|(_, t, _)| **t == *tile) {
            if *backing != Ground::Water {
                commands.entity(entity).insert(Ground::Ground(60));
            }
        }
    }
}

fn plant_flowers_in_tiles(
    query: Query<(Entity, &Tile, &Ground)>,
    mut plant_flower: EventReader<PlantFlower>,
    mut commands: Commands,
    plants: Res<PlantDefinitions>,
) {
    let flower_id = plants
        .name_to_id
        .get("flower")
        .expect("Flower isn't loaded");
    for plant_flower in plant_flower.iter() {
        let tile = &plant_flower.0;
        if let Some((entity, _, backing)) = query.iter().find(|(_, t, _)| **t == *tile) {
            if *backing != Ground::Water {
                commands.entity(entity).insert(Plant::Plant(*flower_id));
            }
        }
    }
}

fn update_tiles(
    query: Query<(Entity, &Ground, &Plant, &Tile)>,
    mut commands: Commands,
    plants: Res<PlantDefinitions>,
) {
    let tiles = query
        .iter()
        .map(|(_, b, c, t)| (*t, (b, c)))
        .collect::<HashMap<_, _>>();

    for (entity, ground, plant, tile) in query.iter() {
        let new_ground = update_backing(ground, plant, tile, &tiles, &plants.definitions);
        let new_plant = update_cell(&new_ground, plant, tile, &tiles, &plants.definitions);

        if new_ground != *ground {
            commands.entity(entity).insert(new_ground);
        }

        if new_plant != *plant {
            commands.entity(entity).insert(new_plant);
        }
    }
}

fn update_cell(
    ground: &Ground,
    plant: &Plant,
    tile: &Tile,
    tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plant)>,
    plants: &[PlantDefinition],
) -> Plant {
    match (ground, plant) {
        (Ground::Water, _) => Plant::Empty,
        (Ground::Ground(nutrients), Plant::Empty) => {
            let plant = plants.iter().enumerate().find(|(id, p)| {
                if p.spread_threshold <= *nutrients {
                    if p.seeded {
                        let count = count_matching_neighbours(tile, tiles, |(_, p)| {
                            **p == Plant::Plant(*id)
                        });
                        let result = count > 0;
                        info!("Seeded Spread Count: {count} - for id {id} - {result}");
                        result
                    } else {
                        true
                    }
                } else {
                    false
                }
            });
            match plant {
                Some((id, _)) => Plant::Plant(id),
                None => Plant::Empty,
            }
        }
        (Ground::Ground(nutrients), Plant::Plant(id)) => {
            let plant = plants.iter().enumerate().find(|(i, p)| {
                if i != id && p.spread_threshold <= *nutrients {
                    if p.seeded {
                        let count = count_matching_neighbours(tile, tiles, |(_, p)| {
                            **p == Plant::Plant(*i)
                        });
                        let result = count > 0;
                        info!("Seeded Spread Count: {count} - for id {id} - {result}");
                        result
                    } else {
                        true
                    }
                } else {
                    i == id && p.survive_threshold <= *nutrients
                }
            });
            match plant {
                Some((id, _)) => Plant::Plant(id),
                None => Plant::Empty,
            }
        }
    }
}

fn update_backing(
    ground: &Ground,
    plant: &Plant,
    tile: &Tile,
    tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plant)>,
    plants: &[PlantDefinition],
) -> Ground {
    match ground {
        Ground::Water => Ground::Water,
        Ground::Ground(nutrients) => {
            let nutrients = *nutrients as i8;
            let available_nutrients = nutrients.saturating_sub(
                plant
                    .definition(plants)
                    .map(|p| p.local_cost)
                    .unwrap_or_default(),
            );
            let neighbour_nutrients =
                process_neighbours(tile, tiles, available_nutrients, |value, (g, p)| {
                    if let Ground::Ground(nutrients) = **g {
                        let nutrients = nutrients as i8;
                        let available_nutrients = nutrients.saturating_sub(
                            p.definition(plants)
                                .map(|p| p.neighbour_cost)
                                .unwrap_or_default(),
                        );
                        value + available_nutrients
                    } else {
                        value + 16
                    }
                });

            let mut nutrients = neighbour_nutrients / 9;

            nutrients = nutrients.max(0).min(8);
            Ground::Ground(nutrients as u8)
        }
    }
}

const NEIGHBOURHOOD: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_matching_neighbours<T>(
    tile: &Tile,
    tiles: &HashMap<Tile, T>,
    f: impl Fn(&T) -> bool,
) -> u8 {
    NEIGHBOURHOOD
        .iter()
        .map(|(x, y)| Tile(tile.0 + *x, tile.1 + *y))
        .filter_map(|t| tiles.get(&t))
        .fold(0, |value, tile| if f(tile) { value + 1 } else { value })
}

fn process_neighbours<T, R>(
    tile: &Tile,
    tiles: &HashMap<Tile, T>,
    initial: R,
    f: impl Fn(R, &T) -> R,
) -> R {
    NEIGHBOURHOOD
        .iter()
        .map(|(x, y)| Tile(tile.0 + *x, tile.1 + *y))
        .filter_map(|t| tiles.get(&t))
        .fold(initial, f)
}
