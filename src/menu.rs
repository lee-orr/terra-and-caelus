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
    commands.spawn((
        MenuItem,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Terra and Caelus",
            TextStyle {
                font: assets.font.clone(),
                font_size: 100.0,
                color: colors::LIGHT,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    ));
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
