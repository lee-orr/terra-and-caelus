mod assets;
mod colors;
mod control;
mod credits;
mod display;
mod generate_tiles;
mod level_asset;
mod level_complete_screen;
mod level_list;
mod level_loading_screen;
mod level_ui;
mod loading_screen;
mod menu;
mod states;
mod target;
mod tile;
mod update_tiles;

use assets::GameAssets;
use belly::prelude::BellyPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use control::ControlPlugin;
use credits::CreditsPlugin;
use display::TileDisplayPlugin;
use generate_tiles::TileGeneratorPlugin;
use level_asset::LevelAssetPlugin;
use level_complete_screen::LevelCompleteScreenPlugin;
use level_list::LevelListPlugin;
use level_loading_screen::LevelLoadingScreenPlugin;
use level_ui::LevelUiPlugin;
use loading_screen::LoadingScreenPlugin;
use menu::MenuPlugin;
use states::AppState;
use target::TargetPlugin;
use tile::{PlantDefinitions, TilePlugin};
use update_tiles::UpdateTilesPlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        // Resource Setup
        .add_state::<AppState>()
        .insert_resource(ClearColor(colors::BACKGROUND))
        // Built In Plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        // External Plugins
        .add_plugin(Shape2dPlugin::default())
        .add_plugin(BellyPlugin)
        .add_plugin(
            ResourceInspectorPlugin::<PlantDefinitions>::default()
                .run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::Menu),
        )
        .add_collection_to_loading_state::<_, GameAssets>(AppState::LoadingAssets)
        .init_resource_after_loading_state::<_, PlantDefinitions>(AppState::LoadingAssets)
        // Internal Plugins
        .add_plugin(LevelAssetPlugin)
        .add_plugin(LoadingScreenPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LevelListPlugin)
        .add_plugin(LevelLoadingScreenPlugin)
        .add_plugin(LevelCompleteScreenPlugin)
        .add_plugin(LevelUiPlugin)
        .add_plugin(CreditsPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(TileGeneratorPlugin)
        .add_plugin(TileDisplayPlugin)
        .add_plugin(UpdateTilesPlugin)
        .add_plugin(ControlPlugin)
        .add_plugin(TargetPlugin)
        // Systems
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
