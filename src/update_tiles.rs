use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    states::AppState,
    tile::{Fertalize, Ground, Plants, Tile},
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
                commands.entity(entity).insert(Ground::Ground(8));
            }
        }
    }
}

fn update_tiles(query: Query<(Entity, &Ground, &Plants, &Tile)>, mut commands: Commands) {
    let tiles = query
        .iter()
        .map(|(_, b, c, t)| (*t, (b, c)))
        .collect::<HashMap<_, _>>();

    for (e, b, c, t) in query.iter() {
        let nb = update_backing(b, t, &tiles);
        let (nb, nc) = update_cell(&nb, c, t, &tiles);

        if nb != *b {
            commands.entity(e).insert(nb);
        }

        if nc != *c {
            commands.entity(e).insert(nc);
        }
    }
}

fn update_cell(
    b: &Ground,
    c: &Plants,
    t: &Tile,
    tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plants)>,
) -> (Ground, Plants) {
    match (b, c) {
        (Ground::Water, _) => (*b, *c),
        (Ground::Ground(fertility), Plants::Empty) => {
            let plant_count = count_matching_neighbours(t, tiles, |(_, c)| **c != Plants::Empty);
            if plant_count > (8 - *fertility) {
                (*b, Plants::Moss)
            } else {
                (*b, *c)
            }
        }
        (Ground::Ground(fertility), Plants::Moss) => {
            let plant_count = count_matching_neighbours(t, tiles, |(_, c)| **c != Plants::Empty);
            if *fertility == 0 {
                (*b, Plants::Empty)
            } else if plant_count > (8 - *fertility) {
                (*b, Plants::Flowers)
            } else {
                (*b, *c)
            }
        }
        (Ground::Ground(fertility), Plants::Flowers) => {
            if *fertility < 3 {
                (*b, Plants::Moss)
            } else {
                (*b, *c)
            }
        }
    }
}

fn update_backing(
    b: &Ground,
    t: &Tile,
    tiles: &bevy::utils::hashbrown::HashMap<Tile, (&Ground, &Plants)>,
) -> Ground {
    match b {
        Ground::Water => *b,
        Ground::Ground(u) => {
            let mut u = *u;
            let water_count = count_matching_neighbours(t, tiles, |(b, _)| **b == Ground::Water);
            let moss_count = count_matching_neighbours(t, tiles, |(_, c)| **c == Plants::Moss);
            let flower_count = count_matching_neighbours(t, tiles, |(_, c)| **c == Plants::Flowers);

            u += water_count;
            u = u.saturating_sub(moss_count / 2);
            u = u.saturating_sub(flower_count);

            u = u.max(water_count).min(8);
            Ground::Ground(u)
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

fn count_matching_neighbours<T>(tile: &Tile, map: &HashMap<Tile, T>, f: impl Fn(&T) -> bool) -> u8 {
    NEIGHBOURHOOD
        .iter()
        .map(|(x, y)| Tile(tile.0 + *x, tile.1 + *y))
        .filter_map(|t| map.get(&t))
        .fold(0, |value, tile| if f(tile) { value + 1 } else { value })
}
