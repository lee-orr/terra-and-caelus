use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    states::AppState,
    tile::{Fertalize, PlantFlower, TILE_WORLD_SIZE},
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_set(OnUpdate(AppState::InGame)))
            .add_system(show_position.in_set(OnUpdate(AppState::InGame)))
            .add_system(click.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Player(pub i8, pub i8);

fn setup(
    mut commands: Commands,
    players: Query<(Entity, &Player), Without<Sprite>>,
    assets: Res<GameAssets>,
) {
    for (e, player) in players.iter() {
        commands.entity(e).insert(SpriteBundle {
            transform: Transform::from_translation(
                Vec3::new(player.0 as f32, player.1 as f32, 2.) * TILE_WORLD_SIZE,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                ..default()
            },
            texture: assets.player.clone(),
            ..default()
        });
    }
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
    mut plant_flower: EventWriter<PlantFlower>,
) {
    if !buttons.just_pressed(MouseButton::Left) && !buttons.just_pressed(MouseButton::Right) {
        return;
    }

    let (Ok(window), Ok((camera, camera_transform))) = (window.get_single(), camera_q.get_single()) else { return; };
    let Some(position) = window.cursor_position().and_then(|viewport_position| camera.viewport_to_world(camera_transform, viewport_position)).map(|r| r.origin.truncate()) else { return; };

    let tile_position = (position / TILE_WORLD_SIZE + 0.5).into();

    if buttons.just_pressed(MouseButton::Left) {
        fertilize.send(Fertalize(tile_position));
    } else {
        plant_flower.send(PlantFlower(tile_position));
    }
}
