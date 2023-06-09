use belly::prelude::StyleSheet;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_kira_audio::AudioSource;

use crate::{
    level_asset::LevelList,
    tile::{PlantDefinitions, PlantDefinitionsAsset},
};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "font.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "ui.ess")]
    pub ui_style: Handle<StyleSheet>,

    #[asset(path = "player.png")]
    pub player: Handle<Image>,
    #[asset(path = "goal_pillar.png")]
    pub goal: Handle<Image>,
    #[asset(path = "fire.png")]
    pub fire: Handle<Image>,

    #[asset(path = "card_drain.png")]
    pub card_drain: Handle<Image>,
    #[asset(path = "card_fertilize.png")]
    pub card_fertilize: Handle<Image>,
    #[asset(path = "card_fire.png")]
    pub card_fire: Handle<Image>,
    #[asset(path = "card_seed.png")]
    pub card_seed: Handle<Image>,
    #[asset(path = "card_move.png")]
    pub card_move: Handle<Image>,

    #[asset(path = "shrine_drain.png")]
    pub shrine_drain: Handle<Image>,
    #[asset(path = "shrine_fertilize.png")]
    pub shrine_fertilize: Handle<Image>,
    #[asset(path = "shrine_fire.png")]
    pub shrine_fire: Handle<Image>,
    #[asset(path = "shrine_seed.png")]
    pub shrine_seed: Handle<Image>,

    #[asset(path = "water.png")]
    pub water: Handle<Image>,
    #[asset(path = "void.png")]
    pub void: Handle<Image>,
    #[asset(path = "fertile_ground.png")]
    pub fertile_soil: Handle<Image>,
    #[asset(path = "harsh_ground.png")]
    pub soil: Handle<Image>,
    #[asset(path = "sand.png")]
    pub sand: Handle<Image>,
    #[asset(path = "sand_fertile.png")]
    pub fertile_sand: Handle<Image>,
    #[asset(path = "rock.png")]
    pub rock: Handle<Image>,
    #[asset(path = "rock_fertile.png")]
    pub fertile_rock: Handle<Image>,

    #[asset(path = "flower.png")]
    pub flower: Handle<Image>,
    #[asset(path = "moss.png")]
    pub moss: Handle<Image>,

    #[asset(path = "plants.pdef.json")]
    pub plants: Handle<PlantDefinitionsAsset>,
    #[asset(path = "levels/levels.lvl.list.json")]
    pub levels: Handle<LevelList>,

    #[asset(path = "terra.ogg")]
    pub music: Handle<AudioSource>,
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

        info!("Plant Definitions Asset: {p:#?}");

        let definitions = (p, server.clone()).into();
        info!("Plant Definitions: {definitions:#?}");
        definitions
    }
}
