use std::str::FromStr;

use bevy::{
    prelude::*,
    reflect::{FromReflect, Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::{Deserialize, Serialize};

use crate::assets::GameAssets;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub enum Ground {
    #[default]
    Water,
    Ground(i16),
}

impl FromStr for Ground {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "g" == s {
            Ok(Ground::Ground(8))
        } else {
            Ok(Ground::Water)
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub enum Plant {
    #[default]
    Empty,
    Plant(String),
}

impl Plant {
    pub fn definition<'a>(
        &self,
        plants: &'a [PlantDefinition],
        name_to_id: &'a HashMap<String, usize>,
    ) -> Option<&'a PlantDefinition> {
        match self {
            Plant::Empty => None,
            Plant::Plant(id) => name_to_id.get(id).and_then(|id| plants.get(*id)),
        }
    }
}

impl FromStr for Plant {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('.') {
            let s = s.trim_start_matches('.');
            Ok(Plant::Plant(s.to_string()))
        } else {
            Ok(Plant::Empty)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameEntity {
    Player,
}

impl FromStr for GameEntity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" => Ok(GameEntity::Player),
            _ => Err(anyhow::Error::msg("No Entity")),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub struct Tile(pub i8, pub i8);

impl From<Vec2> for Tile {
    fn from(value: Vec2) -> Self {
        Tile(value.x.floor() as i8, value.y.floor() as i8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fertalize(pub Tile);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlantFlower(pub Tile);

pub const TILE_WORLD_SIZE: f32 = 40.;

#[derive(
    Debug,
    Default,
    Clone,
    Reflect,
    PartialEq,
    FromReflect,
    Serialize,
    Deserialize,
    TypeUuid,
    InspectorOptions,
)]
#[uuid = "11c21cdc-4ee1-4112-94fe-645868914bc2"]
pub struct PlantDefinition {
    pub color: Color,
    pub seeded: bool,
    pub aggressiveness: u8,
    pub survive_threshold: i16,
    pub spread_threshold: i16,
    pub local_cost: i16,
    pub neighbour_cost: i16,
    pub asset: String,
    pub id: String,
}

impl PartialOrd for PlantDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.aggressiveness.partial_cmp(&other.aggressiveness) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.seeded.partial_cmp(&other.seeded) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.spread_threshold.partial_cmp(&other.spread_threshold) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.survive_threshold.partial_cmp(&other.survive_threshold) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.local_cost.partial_cmp(&other.local_cost) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.neighbour_cost.partial_cmp(&other.neighbour_cost)
    }
}

#[derive(Debug, Default, Clone, Reflect, PartialEq, FromReflect, TypeUuid, InspectorOptions)]
#[uuid = "11c21cdc-4ee1-4112-94fe-676868914bc2"]
pub struct TileAsset(pub Handle<Image>, pub Color);

#[derive(Debug, Clone, Reflect, Resource, FromReflect, TypeUuid, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
#[uuid = "b17dc730-beba-4e73-89c7-c6cfc692f02a"]
pub struct PlantDefinitions {
    pub definitions: Vec<PlantDefinition>,
    pub name_to_id: HashMap<String, usize>,
    pub assets: HashMap<String, TileAsset>,
}

#[derive(Default, Debug, Clone, TypeUuid, Serialize, Deserialize)]
#[uuid = "b17dc730-beba-4e73-89c7-c6cfc692f02e"]
pub struct PlantDefinitionsAsset(pub Vec<PlantDefinition>);

impl From<(PlantDefinitionsAsset, AssetServer)> for PlantDefinitions {
    fn from((p, server): (PlantDefinitionsAsset, AssetServer)) -> Self {
        let mut p =
            p.0.iter()
                .enumerate()
                .map(|(id, plant)| {
                    let plant = (*plant).clone();
                    (id, plant.id.clone(), plant)
                })
                .collect::<Vec<_>>();

        p.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        Self {
            definitions: p.iter().map(|(_, _, p)| p.clone()).collect(),
            name_to_id: p
                .iter()
                .map(|(id, name, _)| (name.to_string(), *id))
                .collect(),
            assets: p
                .iter()
                .map(|(_, _, p)| (p.id.clone(), TileAsset(server.load(&p.asset), p.color)))
                .collect(),
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Fertalize>()
            .add_event::<PlantFlower>()
            .register_type::<PlantDefinition>()
            .register_type::<PlantDefinitions>()
            .add_plugin(JsonAssetPlugin::<PlantDefinitionsAsset>::new(&[
                "pdef.json",
            ]))
            .add_system(update_plant_definitions);
    }
}

fn update_plant_definitions(
    events: EventReader<AssetEvent<PlantDefinitionsAsset>>,
    server: Res<AssetServer>,
    assets: Option<Res<GameAssets>>,
    plant_assets: Res<Assets<PlantDefinitionsAsset>>,
    mut commands: Commands,
) {
    if events.is_empty() {
        return;
    }
    let Some(assets) = assets else { return; };
    let Some(definitions) = plant_assets.get(&assets.plants) else { return; };
    let server: AssetServer = server.clone();
    let def = (definitions.clone(), server).into();
    commands.insert_resource::<PlantDefinitions>(def);
}
