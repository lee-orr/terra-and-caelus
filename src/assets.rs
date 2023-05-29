use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
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
}
