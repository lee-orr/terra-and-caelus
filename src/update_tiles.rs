use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    states::AppState,
    tile::{Backing, Cell, Tile},
};

pub struct UpdateTilesPlugin;

impl Plugin for UpdateTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_tiles.run_if(
                in_state(AppState::InGame).and_then(on_timer(Duration::from_secs_f32(1.5))),
            ),
        );
    }
}

fn update_tiles(query: Query<(Entity, &Backing, &Cell, &Tile)>, mut commands: Commands) {
    let tiles = query
        .iter()
        .map(|(_, b, c, t)| (*t, (b, c)))
        .collect::<HashMap<_, _>>();

    for (e, b, c, t) in query.iter() {
        let (nb, nc) = match (b, c) {
            (Backing::Empty, Cell::Empty) => {
                let below = Tile(t.0, t.1 - 1);
                if let Some((_, Cell::Moss)) = tiles.get(&below) {
                    (*b, Cell::Moss)
                } else {
                    (*b, *c)
                }
            }
            (Backing::Empty, Cell::Moss) => {
                let below = Tile(t.0, t.1 - 1);
                if let Some((_, Cell::Moss)) = tiles.get(&below) {
                    (*b, Cell::Empty)
                } else {
                    (*b, *c)
                }
            }
            (Backing::Soil, Cell::Empty) => {
                let enough_moss = [-1, 0, 1]
                    .iter()
                    .flat_map(|a| [(a, -1), (a, 0), (a, 1)])
                    .filter(|(x, y)| **x != 0 || *y != 0)
                    .map(|(x, y)| Tile(t.0 + *x, t.1 + y))
                    .filter_map(|t| tiles.get(&t))
                    .fold(0, |a, (_, c)| if **c == Cell::Moss { a + 1 } else { a });

                if enough_moss > 2 {
                    (*b, Cell::Moss)
                } else {
                    (*b, *c)
                }
            }
            (Backing::Soil, Cell::Moss) => {
                let too_much_moss = [-1, 0, 1]
                    .iter()
                    .flat_map(|a| [(a, -1), (a, 0), (a, 1)])
                    .filter(|(x, y)| **x != 0 || *y != 0)
                    .map(|(x, y)| Tile(t.0 + *x, t.1 + y))
                    .filter_map(|t| tiles.get(&t))
                    .fold(0, |a, (_, c)| if **c == Cell::Moss { a + 1 } else { a });

                if too_much_moss > 5 {
                    (*b, Cell::Empty)
                } else {
                    (*b, *c)
                }
            }
        };

        if nb != *b {
            commands.entity(e).insert(nb);
        }

        if nc != *c {
            commands.entity(e).insert(nc);
        }
    }
}
