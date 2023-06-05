use crate::{assets::GameAssets, states::AppState};
use belly::{core::ess::Styles, prelude::*};
use bevy::prelude::*;

pub struct LevelUiPlugin;

impl Plugin for LevelUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::InGame)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, mut styles: ResMut<Styles>) {
    styles.insert(assets.ui_style.clone());
    let ui = commands.spawn(MenuItem).id();

    commands.add(eml! {
        <body {ui} c:root>
                <div c:header>"In Game"</div>
        </body>
    });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
