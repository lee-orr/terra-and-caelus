use crate::{
    assets::GameAssets,
    level_asset::{CurrentLevel, LevelList},
    states::AppState,
};
use belly::prelude::*;
use bevy::prelude::*;

pub struct LevelListPlugin;

impl Plugin for LevelListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::LevelList)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::LevelList)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, level_list: Res<Assets<LevelList>>) {
    let Some(levels) = level_list.get(&assets.levels) else { return; };

    let levels = levels.0.clone();

    let ui = commands.spawn(MenuItem).id();

    commands.add(eml! {
        <body {ui} c:root>
            <div c:header>"Terra and Caelus"</div>
                <for level in=levels>
                    <button c:menu_button on:press=move |ctx| {
                        let level = level.clone();
                        let handle = ctx.load(&level);
                        ctx.commands().insert_resource(CurrentLevel(Some(handle)));
                        ctx.commands().insert_resource(NextState(Some(AppState::LoadingLevel)));
                    }>
                        <span c:content>
                        "Play "{level_display(&level)}
                        </span>
                    </button>
                </for>
            <button c:menu_button c:small_menu_button c:secondary on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::Menu)))>
                <span c:content>
                "Menu"
                </span>
            </button>
        </body>
    });
}

fn level_display(name: &str) -> String {
    name.replace(".lvl.json", "").replace('_', " ")
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
