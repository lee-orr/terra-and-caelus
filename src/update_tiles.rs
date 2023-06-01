use std::{ops::Div, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use petgraph::visit::EdgeRef;

use crate::{
    states::AppState,
    tile::{
        CellData, CellId, CurrentGraph, Fertalize, Graph, Ground, Plant, PlantDefinition,
        PlantDefinitions, PlantFlower,
    },
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

fn fertilize_tiles(graph: Option<ResMut<CurrentGraph>>, mut fertilize: EventReader<Fertalize>) {
    let Some(mut graph) = graph else { return; };

    for fertilize in fertilize.iter() {
        let tile = &fertilize.0;
        let Some(id) = graph.find(*tile) else { error!("No Node To Fertilize {tile:?}, {graph:?}"); continue; };
        let graph = graph.0.get_graph_mut();
        if let Some(n) = graph.node_weight_mut(id) {
            if n.data.0 != Ground::Water {
                n.data = (Ground::Ground(60), n.data.1);
            }
        }
    }
}

fn plant_flowers_in_tiles(
    graph: Option<ResMut<CurrentGraph>>,
    mut plant_flower: EventReader<PlantFlower>,
    plants: Res<PlantDefinitions>,
) {
    let Some(mut graph) = graph else { return; };
    let flower_id = plants
        .name_to_id
        .get("flower")
        .expect("Flower isn't loaded");
    for plant_flower in plant_flower.iter() {
        let tile = &plant_flower.0;
        let Some(id) =graph.find(*tile) else { error!("No Node To Plant {tile:?}, {graph:?}"); continue; };
        let graph = graph.0.get_graph_mut();
        if let Some(n) = graph.node_weight_mut(id) {
            if n.data.0 != Ground::Water {
                n.data = (n.data.0, Plant::Plant(*flower_id));
            }
        }
    }
}

fn update_tiles(
    query: Query<(Entity, &CellId)>,
    mut commands: Commands,
    plants: Res<PlantDefinitions>,
    graph: Option<ResMut<CurrentGraph>>,
) {
    let Some(mut graph) = graph else { return; };
    let source_graph = graph.0.get_graph();
    let mut update_graph = source_graph.clone();

    for (entity, id) in query.iter() {
        let Some(n) = source_graph.node_weight(id.0) else { continue;};
        let (ground, plant) = n.data;

        let Some(new_ground) = update_backing(id, source_graph, &plants.definitions) else { continue; };
        let Some(new_plant) = update_cell(id, source_graph, &plants.definitions) else { continue; };

        if let Some(n) = update_graph.node_weight_mut(id.0) {
            n.data = (new_ground, new_plant);
        }

        if new_ground != ground {
            commands.entity(entity).insert(new_ground);
        }

        if new_plant != plant {
            commands.entity(entity).insert(new_plant);
        }
    }
    graph.0.set_graph(update_graph);
}

fn update_cell(cell: &CellId, graph: &Graph, plants: &[PlantDefinition]) -> Option<Plant> {
    let node = graph.node_weight(cell.0)?;
    let (ground, plant) = node.data;

    Some(match (ground, plant) {
        (Ground::Water, _) => Plant::Empty,
        (Ground::Ground(nutrients), Plant::Empty) => {
            let plant = plants.iter().enumerate().find(|(id, p)| {
                if p.spread_threshold <= nutrients {
                    if p.seeded {
                        let count =
                            count_matching_neighbours(cell, graph, |(_, p)| p == Plant::Plant(*id))
                                .val();
                        count > 0
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
                if *i != id && p.spread_threshold <= nutrients {
                    if p.seeded {
                        let count =
                            count_matching_neighbours(cell, graph, |(_, p)| p == Plant::Plant(*i))
                                .val();
                        count > 0
                    } else {
                        true
                    }
                } else {
                    *i == id && p.survive_threshold <= nutrients
                }
            });
            match plant {
                Some((id, _)) => Plant::Plant(id),
                None => Plant::Empty,
            }
        }
    })
}

fn update_backing(cell: &CellId, graph: &Graph, plants: &[PlantDefinition]) -> Option<Ground> {
    let node = graph.node_weight(cell.0)?;
    let (ground, plant) = node.data;

    match ground {
        Ground::Water => Some(Ground::Water),
        Ground::Ground(nutrients) => {
            let available_nutrients = nutrients.saturating_sub(
                plant
                    .definition(plants)
                    .map(|p| p.local_cost)
                    .unwrap_or_default(),
            );
            let NeighbourProcess {
                num_neighbours,
                value: neighbour_nutrients,
            } = process_neighbours(cell, graph, available_nutrients, |value, (g, p)| {
                if let Ground::Ground(nutrients) = g {
                    let available_nutrients = nutrients.saturating_sub(
                        p.definition(plants)
                            .map(|p| p.neighbour_cost)
                            .unwrap_or_default(),
                    );
                    value.saturating_add(available_nutrients)
                } else {
                    value.saturating_add(16)
                }
            });

            let mut nutrients = neighbour_nutrients.div((num_neighbours + 1) as i16);

            nutrients = nutrients.max(0).min(8);
            Some(Ground::Ground(nutrients))
        }
    }
}

fn count_matching_neighbours(
    cell: &CellId,
    graph: &Graph,
    f: impl Fn(CellData) -> bool,
) -> NeighbourProcess<u8> {
    process_neighbours(
        cell,
        graph,
        0,
        |value, tile| if f(tile) { value + 1 } else { value },
    )
}

struct NeighbourProcess<R> {
    pub num_neighbours: usize,
    pub value: R,
}

impl<R> NeighbourProcess<R> {
    pub fn val(self) -> R {
        self.value
    }
}

fn process_neighbours<R>(
    cell: &CellId,
    graph: &Graph,
    initial: R,
    f: impl Fn(R, CellData) -> R,
) -> NeighbourProcess<R> {
    graph
        .edges(cell.0)
        .filter_map(|edge| {
            let id = if edge.target() == cell.0 {
                edge.source()
            } else {
                edge.target()
            };
            graph.node_weight(id)
        })
        .map(|n| n.data)
        .fold(
            NeighbourProcess {
                num_neighbours: 0,
                value: initial,
            },
            |NeighbourProcess {
                 num_neighbours,
                 value,
             },
             v| {
                NeighbourProcess {
                    num_neighbours: num_neighbours + 1,
                    value: f(value, v),
                }
            },
        )
}
