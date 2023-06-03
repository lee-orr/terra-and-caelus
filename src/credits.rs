use crate::{assets::GameAssets, states::AppState};
use belly::{core::ess::Styles, prelude::*};
use bevy::prelude::*;

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Credits)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::Credits)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, mut styles: ResMut<Styles>) {
    styles.insert(assets.ui_style.clone());
    let ui = commands.spawn(MenuItem).id();

    commands.add(eml! {
        <body {ui} c:root>
            <div c:header>"Terra and Caelus"</div>
            <div c:separator></div>
            <div c:credit>"Design, Development by Lee-Orr"</div>
            <div c:separator></div>
            <div c:subheader>"With Assets By"</div>
            <div c:credit>"Archeolohicaps Font by Manfred Klein"</div>
            <div c:credit>"All Other Art And Music Assets by Lee-Orr"</div>
            <div c:separator></div>
            <div c:subheader>"Using the following Rust crates:"</div>
            <div c:credit>"The Bevy Game Engine"</div>
            <div c:credit>"Noisy Bevy"</div>
            <div c:credit>"Bevy Asset Loader"</div>
            <div c:credit>"Bevy Common Assets"</div>
            <div c:credit>"Serde"</div>
            <div c:credit>"Bevy Vector Shapes"</div>
            <div c:credit>"Belly"</div>
            <div c:credit>"Console Error Panic Hook"</div>
            <div c:credit>"Wasm Bindgen"</div>
            <div c:credit>"Wasm Server Runner"</div>
            <button c:menu_button c:small_menu_button on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::Menu)))>
                <span c:content>
                "Menu"
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
