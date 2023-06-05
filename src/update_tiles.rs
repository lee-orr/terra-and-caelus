use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    control::{AvailablePowers, Power, Seed, UsePower},
    states::AppState,
    tile::{Ground, Plant, PlantDefinition, PlantDefinitions, SpreadType, Tile, FIRE_DURATION},
};

pub struct UpdateTilesPlugin;

impl Plugin for UpdateTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_tiles.run_if(
                in_state(AppState::InGame).and_then(on_timer(Duration::from_secs_f32(0.5))),
            ),
        )
        .add_system(use_powers.in_set(OnUpdate(AppState::InGame)));
    }
}

fn use_powers(
    query: Query<(Entity, &Tile, &Plant, &Ground)>,
    mut use_power: EventReader<UsePower>,
    mut commands: Commands,
    mut powers: ResMut<AvailablePowers>,
    mut seed: ResMut<Seed>,
    plants: Res<PlantDefinitions>,
) {
    let tiles = query
        .iter()
        .map(|(_, t, c, b)| (*t, (b, c)))
        .collect::<HashMap<_, _>>();

    for UsePower(power, tile) in use_power.iter() {
        if let Some((entity, t, plant, ground)) = query.iter().find(|(_, t, _, _)| **t == *tile) {
            match power {
                Power::Fertilize => {
                    powers.adjust(power.clone(), -1);
                    for (entity, tile, _, ground) in query.iter() {
                        if tile.0.abs_diff(t.0) < 2 && tile.1.abs_diff(t.1) < 2 {
                            match ground {
                                Ground::Soil(false) => {
                                    commands.entity(entity).insert(Ground::Soil(true));
                                }
                                Ground::Sand(false) => {
                                    commands.entity(entity).insert(Ground::Sand(true));
                                }
                                Ground::Rock(false) => {
                                    commands.entity(entity).insert(Ground::Rock(true));
                                }
                                _ => {}
                            };
                        }
                    }
                }
                Power::Fire => {
                    if !matches!(ground, Ground::Water)
                        && !matches!(ground, Ground::Empty)
                        && !matches!(plant, Plant::Empty)
                    {
                        commands.entity(entity).insert(Plant::Fire(FIRE_DURATION));
                        powers.adjust(power.clone(), -1);
                    }
                }
                Power::Seed => {
                    if let Plant::Plant(p) = plant {
                        info!("Getting Seed {p}");
                        let Some(asset) = plants.name_to_id.get(p) else {continue;};
                        info!("ID: {asset}");
                        let Some(asset) = plants.definitions.get(*asset) else { continue;};
                        info!("Asset: {asset:?}");

                        seed.0 = Some((p.clone(), asset.asset.clone(), asset.color));
                        powers.adjust(Power::Plant, 1);
                        powers.adjust(power.clone(), -1);
                    }
                }
                Power::Drain => {
                    powers.adjust(power.clone(), -1);
                    for (entity, tile, _, ground) in query.iter() {
                        if tile.0.abs_diff(t.0) < 2 && tile.1.abs_diff(t.1) < 2 {
                            match ground {
                                Ground::Soil(true) => {
                                    commands.entity(entity).insert(Ground::Soil(false));
                                }
                                Ground::Sand(true) => {
                                    commands.entity(entity).insert(Ground::Sand(false));
                                }
                                Ground::Rock(true) => {
                                    commands.entity(entity).insert(Ground::Rock(false));
                                }
                                _ => {}
                            };
                        }
                    }
                }
                Power::Plant => {
                    let Some((plant_id, _, _)) = &seed.0 else { continue; };
                    let Some(plant_id) = plants.name_to_id.get(plant_id) else {continue;};
                    let Some(plant_definition) = plants.definitions.get(*plant_id) else { continue;};
                    if can_survive(plant_definition, ground, plant, tile, &tiles) {
                        commands
                            .entity(entity)
                            .insert(Plant::Plant(plant_definition.id.clone()));
                        powers.adjust(power.clone(), -1);
                        seed.0 = None;
                    }
                }
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
    plant: &Plant,
    tile: &Tile,
    tiles: &HashMap<Tile, (&Ground, &Plant)>,
) -> bool {
    if matches!(plant, Plant::Fire(_)) {
        return false;
    }
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
    if !can_survive(plant_definition, ground, plant, tile, tiles) {
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
    if let Plant::Fire(u) = plant {
        let remaining = u.saturating_sub(1);
        if remaining == 0 {
            return Plant::Empty;
        } else {
            return Plant::Fire(remaining);
        }
    }
    let current_plant = if let Plant::Plant(i) = plant {
        if !matches!(ground, Ground::Water)
            && count_matching_neighbours(tile, tiles, |(_, p)| matches!(p, Plant::Fire(_))) > 0
        {
            return Plant::Fire(FIRE_DURATION);
        }
        i.clone()
    } else {
        "".to_string()
    };

    let plant = plants.iter().find(|p| {
        let i = p.id.as_str();
        if i != current_plant {
            can_spread(p, plant, ground, tile, tiles)
        } else {
            can_survive(p, ground, plant, tile, tiles)
        }
    });

    match plant {
        Some(p) => Plant::Plant(p.id.clone()),
        None => Plant::Empty,
    }
}

fn update_backing(
    ground: &Ground,
    plant: &Plant,
    _tile: &Tile,
    _tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plant)>,
    _plants: &[PlantDefinition],
    _name_to_id: &HashMap<String, usize>,
) -> Ground {
    if matches!(plant, Plant::Fire(_)) {
        match ground {
            Ground::Soil(false) => {
                return Ground::Soil(true);
            }
            Ground::Sand(false) => {
                return Ground::Sand(true);
            }
            Ground::Rock(false) => {
                return Ground::Rock(true);
            }
            _ => {}
        }
    }
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
