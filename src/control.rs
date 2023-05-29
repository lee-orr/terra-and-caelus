use bevy::prelude::*;

use crate::{
    states::AppState,
    tile::{Fertalize, TILE_WORLD_SIZE},
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(show_position.in_set(OnUpdate(AppState::InGame)))
            .add_system(click.in_set(OnUpdate(AppState::InGame)));
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

fn show_position(
    mut query: Query<&mut Transform, With<Player>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (Ok(window), Ok((camera, camera_transform))) = (window.get_single(), camera_q.get_single()) else { return; };
    let Some(position) = window.cursor_position().and_then(|viewport_position| camera.viewport_to_world(camera_transform, viewport_position)).map(|r| r.origin.truncate()) else { return; };
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(position.x, position.y, 5.);
    }
}

fn click(
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut fertilize: EventWriter<Fertalize>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let (Ok(window), Ok((camera, camera_transform))) = (window.get_single(), camera_q.get_single()) else { return; };
    let Some(position) = window.cursor_position().and_then(|viewport_position| camera.viewport_to_world(camera_transform, viewport_position)).map(|r| r.origin.truncate()) else { return; };

    let tile_position = (position / TILE_WORLD_SIZE + 0.5).into();

    fertilize.send(Fertalize(tile_position));
}
