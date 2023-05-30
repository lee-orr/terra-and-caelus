mod assets;
mod control;
mod display;
mod generate_tiles;
mod states;
mod tile;
mod update_tiles;

use assets::GameAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use control::ControlPlugin;
use display::TileDisplayPlugin;
use generate_tiles::TileGeneratorPlugin;
use states::AppState;
use tile::{PlantDefinitions, TilePlugin};
use update_tiles::UpdateTilesPlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, GameAssets>(AppState::LoadingAssets)
        .init_resource_after_loading_state::<_, PlantDefinitions>(AppState::LoadingAssets)
        .add_plugin(TilePlugin)
        .add_plugin(TileGeneratorPlugin)
        .add_plugin(TileDisplayPlugin)
        .add_plugin(UpdateTilesPlugin)
        .add_plugin(ControlPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.2, 0.5)))
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
