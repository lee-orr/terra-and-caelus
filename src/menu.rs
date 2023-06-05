use crate::{assets::GameAssets, states::AppState};
use belly::{core::ess::Styles, prelude::*};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut styles: ResMut<Styles>,
    audio: Res<Audio>,
) {
    styles.insert(assets.ui_style.clone());
    audio.play_with_settings(assets.music.clone(), PlaybackSettings::LOOP);
    let ui = commands.spawn(MenuItem).id();

    commands.add(eml! {
        <body {ui} c:root>
            <div c:header>"Terra and Caelus"</div>
            <div c:subheader>"A Game By Lee-Orr"</div>
            <button c:menu_button on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::LevelList)))>
                <span c:content>
                "Play"
                </span>
            </button>
            <button c:menu_button c:small_menu_button c:secondary on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::Credits)))>
                <span c:content>
                "Credits"
                </span>
            </button>
        </body>
    });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
