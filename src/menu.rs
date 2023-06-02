use bevy::prelude::*;

use crate::{assets::GameAssets, colors, states::AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(clear_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Component)]
struct MenuItem;

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            MenuItem,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::all(Val::Px(10.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(
                TextBundle::from_section(
                    "Terra and Caelus",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: colors::LIGHT,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
            p.spawn(
                TextBundle::from_section(
                    "A Game By Lee-Orr",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.0,
                        color: colors::SECONDARY,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
