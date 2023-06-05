use crate::{assets::GameAssets, states::AppState};
use belly::{core::ess::Styles, prelude::*};
use bevy::prelude::*;

pub struct LevelCompleteScreenPlugin;

impl Plugin for LevelCompleteScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::LevelComplete)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::LevelComplete)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, mut styles: ResMut<Styles>) {
    styles.insert(assets.ui_style.clone());
    let ui = commands.spawn(MenuItem).id();

    commands.add(eml! {
        <body {ui} c:root>
            <div c:panel>
                <div c:header>"Level Complete"</div>
                <button c:menu_button on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::LevelList)))>
                    <span c:content>
                    "Play Another Level"
                    </span>
                </button>
                <button c:menu_button c:small_menu_button on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::Menu)))>
                    <span c:content>
                    "Menu"
                    </span>
                </button>
            </div>
        </body>
    });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
