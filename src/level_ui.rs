use crate::{
    assets::GameAssets,
    control::{AvailablePowers, Seed},
    level_asset::{CurrentLevel, CurrentLevelHotReload, LevelAsset},
    states::AppState,
};
use belly::{core::ess::Styles, prelude::*};
use bevy::prelude::*;

pub struct LevelUiPlugin;

impl Plugin for LevelUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup_menu
                .in_base_set(CoreSet::PreUpdate)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(clear_menu.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Component)]
struct MenuItem;

#[allow(clippy::too_many_arguments)]
fn setup_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut styles: ResMut<Styles>,
    powers: Res<AvailablePowers>,
    seed: Res<Seed>,
    query: Query<Entity, With<MenuItem>>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    let Some(current_level) = current_level.0.as_ref() else { return; };
    let Some(level) = level_assets.get(current_level) else { return;};

    if !powers.is_changed() {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    styles.insert(assets.ui_style.clone());
    let ui = commands.spawn(MenuItem).id();

    let powers = powers
        .0
        .iter()
        .filter_map(|(p, v)| if *v > 0 { Some((p.clone(), *v)) } else { None })
        .collect::<Vec<_>>();
    let seed = seed.clone();
    let level_header = level.name.clone();
    let level_description = level
        .description
        .clone()
        .unwrap_or_default()
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    commands.add(eml! {
        <body {ui} c:in_game>
                <div c:description>
                    <div c:desc_header>
                        <div c:level_header>{level_header}</div>
                        <div>
                        <button c:exit_button on:press=|ctx| ctx.send_event(CurrentLevelHotReload)><span c:content>"Restart"</span></button>
                        <button c:exit_button on:press=|ctx| ctx.commands().insert_resource(NextState(Some(AppState::Menu)))><span c:content>"Exit"</span></button>
                        </div>
                    </div>
                    <for dsc in=level_description>
                        <div c:level_description>{dsc}</div>
                    </for>
                </div>
                <div c:cards>
                    <div c:card c:movement>
                        <img c:card-image src="card_move.png"></img>
                        <span c:key_bind c:up>"W"</span>
                        <span c:key_bind c:down>"S"</span>
                        <span c:key_bind c:left>"A"</span>
                        <span c:key_bind c:right>"D"</span>
                    </div>
                    <for value in=powers>
                        <div class={value.0.ui_class_name()}>
                            <img c:card-image src={value.0.ui_image(&seed).0}></img>
                            <span c:label>{value.0.to_string()}</span>
                            <span c:available>{value.1.to_string()}</span>
                            <span c:key_bind>{value.0.key_binding()}</span>
                        </div>
                    </for>
                </div>
        </body>
    });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
