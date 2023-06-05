use std::{fmt, marker::PhantomData, str::FromStr};

use bevy::{
    prelude::*,
    reflect::{FromReflect, Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::assets::GameAssets;

#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Hash,
    Serialize,
    Deserialize,
    Reflect,
    FromReflect,
)]
pub enum Ground {
    #[default]
    Empty,
    Water,
    Soil(bool),
    Sand(bool),
    Rock(bool),
}

impl FromStr for Ground {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "g" => Ok(Ground::Soil(false)),
            "gf" => Ok(Ground::Soil(true)),
            "s" => Ok(Ground::Sand(false)),
            "sf" => Ok(Ground::Sand(true)),
            "r" => Ok(Ground::Rock(false)),
            "rf" => Ok(Ground::Rock(true)),
            "w" => Ok(Ground::Water),
            _ => Ok(Ground::Empty),
        }
    }
}

impl ToString for Ground {
    fn to_string(&self) -> String {
        match self {
            Ground::Empty => "",
            Ground::Water => "w",
            Ground::Soil(true) => "gf",
            Ground::Sand(true) => "sf",
            Ground::Rock(true) => "rf",
            Ground::Soil(false) => "g",
            Ground::Sand(false) => "s",
            Ground::Rock(false) => "r",
        }
        .to_string()
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
    Target(String),
}

impl FromStr for GameEntity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "p" == s {
            Ok(GameEntity::Player)
        } else if s.starts_with("t.") {
            let s = s.trim_start_matches("t.");
            Ok(GameEntity::Target(s.to_string()))
        } else {
            Err(anyhow::Error::msg("No Entity"))
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

pub const TILE_WORLD_SIZE: f32 = 40.;

#[derive(
    Debug, Clone, Reflect, FromReflect, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum SpreadType {
    AdjacentEmpty(usize),
    AdjacentAggresive(usize),
    AdjacentRequire(usize, Vec<String>),
    Seeded,
    SeededRequire(Vec<String>),
}

impl Default for SpreadType {
    fn default() -> Self {
        Self::AdjacentEmpty(1)
    }
}

#[derive(Debug, Default, Clone, Reflect, FromReflect, PartialEq, Serialize, Deserialize)]
pub struct GroundList(pub Vec<Ground>);

impl FromStr for GroundList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| Ground::from_str(v).unwrap_or_default())
                .collect(),
        ))
    }
}

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
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub spread: SpreadType,
    pub asset: String,
    #[serde(default)]
    pub aggressiveness: usize,
    pub id: String,
    #[serde(deserialize_with = "string_deserializer")]
    pub allowed_grounds: GroundList,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub required_neighbour_grounds: GroundList,
    #[serde(default)]
    pub required_neighbour_plants: Vec<String>,
}

impl PartialOrd for PlantDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.aggressiveness.partial_cmp(&other.aggressiveness) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.spread.partial_cmp(&other.spread) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self
            .allowed_grounds
            .0
            .len()
            .partial_cmp(&other.allowed_grounds.0.len())
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.id.partial_cmp(&other.id)
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

fn string_deserializer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = anyhow::Error>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct Strings<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for Strings<T>
    where
        T: Deserialize<'de> + FromStr<Err = anyhow::Error>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }
    }

    deserializer.deserialize_any(Strings(PhantomData))
}
