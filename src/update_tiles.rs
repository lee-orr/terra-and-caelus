use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    states::AppState,
    tile::{Backing, Cell, Fertalize, Tile},
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
    query: Query<(Entity, &Tile, &Backing)>,
    mut fertilize: EventReader<Fertalize>,
    mut commands: Commands,
) {
    for fertilize in fertilize.iter() {
        let tile = &fertilize.0;
        if let Some((entity, _, backing)) = query.iter().find(|(_, t, _)| **t == *tile) {
            if *backing != Backing::Empty {
                commands.entity(entity).insert(Backing::FertileSoil);
            }
        }
    }
}

fn update_tiles(query: Query<(Entity, &Backing, &Cell, &Tile)>, mut commands: Commands) {
    let tiles = query
        .iter()
        .map(|(_, b, c, t)| (*t, (b, c)))
        .collect::<HashMap<_, _>>();

    for (e, b, c, t) in query.iter() {
        let nb = match b {
            Backing::Empty => *b,
            Backing::FertileSoil => {
                if count_matching_neighbours(t, &tiles, |(b, _)| **b == Backing::DepletedSoil) > 1 {
                    Backing::HarshSoil
                } else {
                    *b
                }
            }
            Backing::HarshSoil => {
                if count_matching_neighbours(t, &tiles, |(b, _)| **b == Backing::DepletedSoil) > 6 {
                    Backing::DepletedSoil
                } else {
                    *b
                }
            }
            Backing::DepletedSoil => *b,
        };
        let (nb, nc) = match (&nb, c) {
            (Backing::Empty, _) => (*b, *c),
            (Backing::FertileSoil, Cell::Empty) => {
                let enough_moss = count_matching_neighbours(t, &tiles, |(_, c)| **c == Cell::Moss);
                if enough_moss > 1 {
                    (*b, Cell::Moss)
                } else {
                    (*b, *c)
                }
            }
            (Backing::FertileSoil, Cell::Moss) => {
                let too_much_moss =
                    count_matching_neighbours(t, &tiles, |(_, c)| **c == Cell::Moss);

                if too_much_moss > 5 {
                    (Backing::HarshSoil, *c)
                } else {
                    (*b, *c)
                }
            }
            (Backing::HarshSoil, Cell::Empty) => {
                let enough_moss = count_matching_neighbours(t, &tiles, |(_, c)| **c == Cell::Moss);
                if enough_moss > 2 {
                    (*b, Cell::Moss)
                } else {
                    (*b, *c)
                }
            }
            (Backing::HarshSoil, Cell::Moss) => {
                let too_much_moss =
                    count_matching_neighbours(t, &tiles, |(_, c)| **c == Cell::Moss);

                if too_much_moss > 3 {
                    (Backing::DepletedSoil, *c)
                } else {
                    (*b, *c)
                }
            }
            (Backing::DepletedSoil, Cell::Empty) => (*b, *c),
            (Backing::DepletedSoil, Cell::Moss) => (*b, Cell::Empty),
        };

        if nb != *b {
            commands.entity(e).insert(nb);
        }

        if nc != *c {
            commands.entity(e).insert(nc);
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
    map: &HashMap<Tile, T>,
    f: impl Fn(&T) -> bool,
) -> u32 {
    NEIGHBOURHOOD
        .iter()
        .map(|(x, y)| Tile(tile.0 + *x, tile.1 + *y))
        .filter_map(|t| map.get(&t))
        .fold(0, |value, tile| if f(tile) { value + 1 } else { value })
}
