use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    assets::GameAssets,
    states::AppState,
    tile::{Fertalize, Ground, Plant, Tile, TILE_WORLD_SIZE},
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system(setup_player.in_set(OnUpdate(AppState::InGame)))
            .add_system(
                move_player
                    .in_set(OnUpdate(AppState::InGame))
                    .before(set_player_position),
            )
            .add_system(set_player_position);
    }
}

#[derive(Component)]
pub struct Player(pub i8, pub i8);

fn setup_player(
    mut commands: Commands,
    players: Query<(Entity, &Player), Without<Sprite>>,
    assets: Res<GameAssets>,
) {
    for (e, player) in players.iter() {
        commands.entity(e).insert((
            SpriteBundle {
                transform: Transform::from_translation(
                    Vec3::new(player.0 as f32, player.1 as f32, 2.) * TILE_WORLD_SIZE,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_WORLD_SIZE, TILE_WORLD_SIZE)),
                    ..default()
                },
                texture: assets.player.clone(),
                ..default()
            },
            InputManagerBundle::<Action> {
                // Stores "which actions are currently pressed"
                action_state: ActionState::default(),
                // Describes how to convert from player inputs into those actions
                input_map: InputMap::new([
                    (UserInput::from(QwertyScanCode::W), Action::Up),
                    (QwertyScanCode::S.into(), Action::Down),
                    (QwertyScanCode::A.into(), Action::Left),
                    (QwertyScanCode::D.into(), Action::Right),
                    (KeyCode::Up.into(), Action::Up),
                    (KeyCode::Down.into(), Action::Down),
                    (KeyCode::Left.into(), Action::Left),
                    (KeyCode::Right.into(), Action::Right),
                    (KeyCode::F.into(), Action::Fertilize),
                ]),
            },
        ));
    }
}

fn set_player_position(mut players: Query<(&Player, &mut Transform), Changed<Player>>) {
    for (p, mut t) in players.iter_mut() {
        t.translation = Vec3::new(p.0 as f32, p.1 as f32, 2.) * TILE_WORLD_SIZE;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Fertilize,
}

fn move_player(
    mut player: Query<(&mut Player, &ActionState<Action>)>,
    query: Query<(&Tile, &Ground, &Plant)>,
    mut fertilize: EventWriter<Fertalize>,
) {
    for (mut p, a) in player.iter_mut() {
        let mut target = Tile(p.0, p.1);
        if a.just_pressed(Action::Up) {
            target.1 += 1;
        }
        if a.just_pressed(Action::Down) {
            target.1 -= 1;
        }
        if a.just_pressed(Action::Left) {
            target.0 -= 1;
        }
        if a.just_pressed(Action::Right) {
            target.0 += 1;
        }
        if query
            .iter()
            .any(|(t, _g, p)| *t == target && !matches!(p, Plant::Empty))
        {
            p.0 = target.0;
            p.1 = target.1;
        }

        if a.just_pressed(Action::Fertilize) {
            fertilize.send(Fertalize(Tile(p.0, p.1)));
        }
    }
}
