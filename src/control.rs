use bevy::prelude::*;

use crate::states::AppState;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(show_position.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 5.),
            ..Default::default()
        },
    ));
}

fn show_position(mut query: Query<&mut Transform, With<Player>>, window: Query<&Window>) {
    let Ok(window) = window.get_single() else { return; };
    let Some(position) = window.cursor_position() else { return; };
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(position.x, position.y, 5.);
    }
}
