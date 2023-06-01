use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use fdg_sim::{ForceGraph, ForceGraphHelper, Node, Simulation, SimulationParameters};
use petgraph::{
    stable_graph::NodeIndex,
    visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences},
};

use crate::{
    states::AppState,
    tile::{
        CellData, CellId, CurrentGraph, Graph, Ground, Plant, PlantDefinitions, TILE_WORLD_SIZE,
    },
};

pub struct TileGeneratorPlugin;

impl Plugin for TileGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(NoisyGenerator(Vec2::new(15.130, 24124.563), 139.524))
            .add_system(generate_tiles.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                generate_tiles
                    .in_base_set(CoreSet::PostUpdate)
                    .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::F1))),
            );
    }
}

#[derive(Resource)]
pub struct NoisyGenerator(Vec2, f32);

// impl NoisyGenerator {
//     pub fn generate_value(&mut self) -> f32 {
//         let x = simplex_noise_2d_seeded(self.0, self.1);
//         self.0 = Vec2::new(
//             self.0.y * x.div(self.1) + self.0.x,
//             self.1 * self.1 - self.0.x.div(self.0.y),
//         );
//         x
//     }

//     pub fn select_option<'a, T>(&mut self, options: &'a [T]) -> Option<&'a T> {
//         if options.is_empty() {
//             return None;
//         }
//         let value = self.generate_value();
//         let index = (options.len() as f32 * value).abs().floor() as usize;
//         options.get(index)
//     }
// }

#[derive(Component)]
pub struct Level;

fn generate_graph(
    nodes: &Graph,
    _generator: &mut NoisyGenerator,
    plants: &PlantDefinitions,
) -> ForceGraph<(Ground, Plant), ()> {
    let moss_id = *(plants.name_to_id.get("moss").expect("Moss isn't loaded"));
    let mut nodes: Graph = nodes.clone();
    if nodes.node_count() == 0 {
        let id = nodes.add_force_node_with_coords(
            "a",
            (Ground::Ground(8), Plant::Plant(moss_id)),
            (0., 0., 0.).into(),
        );
        let id_2 = nodes.add_force_node_with_coords(
            "b",
            (Ground::Ground(8), Plant::Empty),
            (TILE_WORLD_SIZE, 0., 0.).into(),
        );
        nodes.add_edge(id, id_2, ());
    }
    nodes
}

fn build_tile(p: &mut ChildBuilder, id: NodeIndex, node: &Node<CellData>) {
    let (ground, plant) = node.data;
    let (x, y) = (node.location.x, node.location.y);
    p.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        },
        ground,
        plant,
        CellId(id),
    ))
    .with_children(|p| {
        p.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                ..default()
            },
            ..default()
        });
    });
}

fn generate_tiles(
    mut commands: Commands,
    existing_levels: Query<Entity, With<Level>>,
    mut generator: ResMut<NoisyGenerator>,
    plants: Res<PlantDefinitions>,
) {
    for entity in existing_levels.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let graph = generate_graph(&ForceGraph::default(), generator.as_mut(), plants.as_ref());

    let simulation = layout_graph(graph);

    commands
        .spawn((SpatialBundle::default(), Level))
        .with_children(|p| {
            for (id, node) in simulation.get_graph().node_references() {
                build_tile(p, id, node);
            }
        });

    commands.insert_resource(CurrentGraph(simulation));
}

const MIN_DIST: f32 = TILE_WORLD_SIZE * 1.1;

fn layout_graph(
    graph: petgraph::stable_graph::StableGraph<Node<(Ground, Plant)>, (), petgraph::Undirected>,
) -> Simulation<(Ground, Plant), ()> {
    let mut simulation = Simulation::from_graph(graph, SimulationParameters::default());

    for _i in 0..50 {
        simulation.update(0.3);
    }

    let graph = simulation.get_graph_mut();

    let min_edge_len = graph
        .edge_references()
        .filter_map(|e| {
            let a = graph.node_weight(e.source())?;
            let b = graph.node_weight(e.target())?;
            Some(a.location.distance(b.location))
        })
        .fold(f32::MAX, |a, b| a.abs().min(b.abs()));

    if min_edge_len < MIN_DIST && min_edge_len > 0. {
        let factor = MIN_DIST / min_edge_len;
        for n in graph.node_weights_mut() {
            n.location = factor * n.location;
        }
    }

    simulation
}
