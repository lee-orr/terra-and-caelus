use belly::prelude::StyleSheet;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::tile::{PlantDefinitions, PlantDefinitionsAsset, TileAsset};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "font.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "ui.ess")]
    pub ui_style: Handle<StyleSheet>,

    #[asset(path = "water.png")]
    pub water: Handle<Image>,
    #[asset(path = "fertile_ground.png")]
    pub fertile_ground: Handle<Image>,
    #[asset(path = "harsh_ground.png")]
    pub harsh_ground: Handle<Image>,
    #[asset(path = "depleted_ground.png")]
    pub depleted_ground: Handle<Image>,
    #[asset(path = "flower.png")]
    pub flower: Handle<Image>,
    #[asset(path = "moss.png")]
    pub moss: Handle<Image>,

    #[asset(path = "plants.pdef.json")]
    pub plants: Handle<PlantDefinitionsAsset>,
}

impl FromWorld for PlantDefinitions {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<GameAssets>()
            .expect("Failed to load assets.");
        let server = cell
            .get_resource::<AssetServer>()
            .expect("Asset server doesn't exist");
        let plant_definitions = cell
            .get_resource_mut::<Assets<PlantDefinitionsAsset>>()
            .expect("No Plant Definitions");

        let p = plant_definitions
            .get(&assets.plants)
            .cloned()
            .unwrap_or_default();

        info!("Plant Definitions: {p:?}");

        let mut p = p.0.iter().collect::<Vec<_>>();
        p.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        let p = p
            .iter()
            .enumerate()
            .map(|(id, (name, plant))| (id, name, plant))
            .collect::<Vec<_>>();

        Self {
            definitions: p.iter().map(|(_, _, p)| (**p).clone()).collect(),
            name_to_id: p
                .iter()
                .map(|(id, name, _)| (name.to_string(), *id))
                .collect(),
            assets: p
                .iter()
                .map(|(_, _, p)| TileAsset(server.load(&p.asset), p.color))
                .collect(),
        }
    }
}
