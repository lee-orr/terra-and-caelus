use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    states::AppState,
    tile::{Fertalize, Ground, Plant, PlantDefinition, PlantDefinitions, SpreadType, Tile},
};

pub struct UpdateTilesPlugin;

impl Plugin for UpdateTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_tiles.run_if(
                in_state(AppState::InGame).and_then(on_timer(Duration::from_secs_f32(0.5))),
            ),
        )
        .add_system(fertilize_tiles.in_set(OnUpdate(AppState::InGame)));
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
                commands.entity(entity).insert(Ground::Soil(true));
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
        let new_ground = update_backing(
            ground,
            plant,
            tile,
            &tiles,
            &plants.definitions,
            &plants.name_to_id,
        );
        let new_plant = update_plant(&new_ground, plant, tile, &tiles, &plants.definitions);

        if new_ground != *ground {
            commands.entity(entity).insert(new_ground);
        }

        if new_plant != *plant {
            commands.entity(entity).insert(new_plant);
        }
    }
}

fn can_survive(
    plant_definition: &PlantDefinition,
    ground: &Ground,
    tile: &Tile,
    tiles: &HashMap<Tile, (&Ground, &Plant)>,
) -> bool {
    if !plant_definition.allowed_grounds.0.contains(ground) {
        return false;
    }
    if !plant_definition.required_neighbour_grounds.0.is_empty()
        && count_matching_neighbours(tile, tiles, |(g, _)| {
            plant_definition.required_neighbour_grounds.0.contains(*g)
        }) == 0
    {
        return false;
    }
    if !plant_definition.required_neighbour_plants.is_empty()
        && count_matching_neighbours(tile, tiles, |(_, p)| {
            if let Plant::Plant(p) = p {
                plant_definition.required_neighbour_plants.contains(p)
            } else {
                false
            }
        }) == 0
    {
        return false;
    }

    true
}

fn can_spread(
    plant_definition: &PlantDefinition,
    plant: &Plant,
    ground: &Ground,
    tile: &Tile,
    tiles: &HashMap<Tile, (&Ground, &Plant)>,
) -> bool {
    if !can_survive(plant_definition, ground, tile, tiles) {
        return false;
    }

    match &plant_definition.spread {
        SpreadType::AdjacentEmpty(n) => {
            *plant == Plant::Empty
                && count_matching_neighbours(tile, tiles, |(_, p)| {
                    if let Plant::Plant(p) = p {
                        *p == plant_definition.id
                    } else {
                        false
                    }
                }) >= *n
        }
        SpreadType::AdjacentAggresive(n) => {
            count_matching_neighbours(tile, tiles, |(_, p)| {
                if let Plant::Plant(p) = p {
                    *p == plant_definition.id
                } else {
                    false
                }
            }) >= *n
        }
        SpreadType::AdjacentRequire(n, req) => {
            count_matching_neighbours(tile, tiles, |(_, p)| {
                if let Plant::Plant(p) = p {
                    req.contains(p)
                } else {
                    false
                }
            }) >= 1
                && count_matching_neighbours(tile, tiles, |(_, p)| {
                    if let Plant::Plant(p) = p {
                        *p == plant_definition.id
                    } else {
                        false
                    }
                }) >= *n
        }
        _ => false,
    }
}

fn update_plant(
    ground: &Ground,
    plant: &Plant,
    tile: &Tile,
    tiles: &HashMap<Tile, (&Ground, &Plant)>,
    plants: &[PlantDefinition],
) -> Plant {
    let current_plant = if let Plant::Plant(i) = plant {
        i.clone()
    } else {
        "".to_string()
    };

    let plant = plants.iter().find(|p| {
        let i = p.id.as_str();
        if i != current_plant {
            can_spread(p, plant, ground, tile, tiles)
        } else {
            can_survive(p, ground, tile, tiles)
        }
    });

    match plant {
        Some(p) => Plant::Plant(p.id.clone()),
        None => Plant::Empty,
    }
}

fn update_backing(
    ground: &Ground,
    _plant: &Plant,
    _tile: &Tile,
    _tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plant)>,
    _plants: &[PlantDefinition],
    _name_to_id: &HashMap<String, usize>,
) -> Ground {
    *ground
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
) -> usize {
    process_neighbours(
        tile,
        tiles,
        0,
        |value, tile| if f(tile) { value + 1 } else { value },
    )
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
