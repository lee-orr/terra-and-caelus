use bevy::{
    prelude::*,
    reflect::{FromReflect, Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Ground {
    #[default]
    Water,
    Ground(i16),
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Plant {
    #[default]
    Empty,
    Plant(usize),
}

impl Plant {
    pub fn definition<'a>(&self, plants: &'a [PlantDefinition]) -> Option<&'a PlantDefinition> {
        match self {
            Plant::Empty => None,
            Plant::Plant(id) => plants.get(*id),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
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
    pub assets: Vec<TileAsset>,
}

#[derive(Default, Debug, Clone, TypeUuid, Serialize, Deserialize)]
#[uuid = "b17dc730-beba-4e73-89c7-c6cfc692f02e"]
pub struct PlantDefinitionsAsset(pub HashMap<String, PlantDefinition>);

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Fertalize>()
            .add_event::<PlantFlower>()
            .register_type::<PlantDefinition>()
            .register_type::<PlantDefinitions>()
            .add_plugin(JsonAssetPlugin::<PlantDefinitionsAsset>::new(&[
                "pdef.json",
            ]));
    }
}
