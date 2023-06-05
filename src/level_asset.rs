use std::{fmt, marker::PhantomData, str::FromStr};

use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::{json::JsonAssetPlugin, yaml::YamlAssetPlugin};
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::tile::{GameEntity, Ground, Plant, Tile};

pub struct LevelAssetPlugin;

impl Plugin for LevelAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(YamlAssetPlugin::<LevelAsset>::new(&["lvl.yaml"]))
            .add_plugin(JsonAssetPlugin::<LevelList>::new(&["lvl.list.json"]))
            .init_resource::<CurrentLevel>()
            .add_event::<CurrentLevelHotReload>()
            .add_system(reload_current_level);
    }
}

#[derive(Clone, Serialize, Deserialize, TypeUuid)]
#[uuid = "afc86691-e9b3-45d0-9b2d-109427c3bc18"]
pub struct LevelList(pub Vec<String>);

#[derive(Clone, Serialize, Deserialize, TypeUuid, Default)]
#[uuid = "8301b47f-95b1-43b0-b4c3-32e45faa0f2f"]
pub struct LevelAsset {
    #[serde(deserialize_with = "strings_or_struct")]
    pub tiles: LevelTiles,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct CurrentLevel(pub Option<Handle<LevelAsset>>);

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LevelTiles(pub HashMap<Tile, (Ground, Plant, Vec<GameEntity>)>);

impl FromStr for LevelTiles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max_y = 0;
        let mut max_x = 0;

        let mut tiles = vec![];
        for (y, line) in s.lines().enumerate() {
            max_y = max_y.max(y);
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            for (x, tile) in line.split_whitespace().enumerate() {
                max_x = max_x.max(x);
                let mut split = tile.split(':');
                let (ground, plant) = (split.next(), split.next());
                let tile = Tile(x as i8, y as i8);

                let plant = plant
                    .map(|p: &str| Plant::from_str(p).unwrap_or_default())
                    .unwrap_or_default();

                let ground = ground
                    .map(|g| Ground::from_str(g).unwrap_or_default())
                    .map(|g| {
                        if matches!(plant, Plant::Plant(_)) {
                            match g {
                                Ground::Soil(_) => Ground::Soil(true),
                                Ground::Sand(_) => Ground::Sand(true),
                                Ground::Rock(_) => Ground::Rock(true),
                                _ => g,
                            }
                        } else {
                            g
                        }
                    })
                    .unwrap_or_default();

                let content = (
                    ground,
                    plant,
                    split.filter_map(|s| GameEntity::from_str(s).ok()).collect(),
                );

                tiles.push((tile, content));
            }
        }

        let y_offset = max_y as i8 / 2;
        let x_offset = max_x as i8 / 2;

        let tiles = tiles
            .into_iter()
            .map(|(Tile(x, y), value)| (Tile(x - x_offset, y_offset - y), value))
            .collect();

        Ok(Self(tiles))
    }
}

fn strings_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = anyhow::Error>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringsOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringsOrStruct<T>
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

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringsOrStruct(PhantomData))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentLevelHotReload;

fn reload_current_level(
    mut ev_asset: EventReader<AssetEvent<LevelAsset>>,
    current_level: Res<CurrentLevel>,
    mut ev_writer: EventWriter<CurrentLevelHotReload>,
) {
    let Some(current) = current_level.0.as_ref() else {
        return;
    };
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Modified { handle } if handle == current => {
                ev_writer.send(CurrentLevelHotReload);
            }
            _ => {}
        }
    }
}
