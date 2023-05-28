mod display;
mod generate_tiles;
mod states;
mod tile;
mod update_tiles;

use bevy::prelude::*;
use bevy_proto::prelude::ProtoPlugin;
use display::TileDisplayPlugin;
use generate_tiles::TileGeneratorPlugin;
use states::AppState;
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
        .add_plugin(ProtoPlugin::new())
        .add_plugin(TileGeneratorPlugin)
        .add_plugin(TileDisplayPlugin)
        .add_plugin(UpdateTilesPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.2, 0.5)))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
