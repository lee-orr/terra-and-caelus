use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::*;

use crate::{
    assets::GameAssets,
    generate_tiles::LevelLoaded,
    states::AppState,
    tile::{Fertalize, Ground, Plant, Tile, TILE_WORLD_SIZE},
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvailablePowers>()
            .add_event::<GainPower>()
            .add_event::<UsePower>()
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_system(setup_player.in_set(OnUpdate(AppState::InGame)))
            .add_system(
                move_player
                    .in_set(OnUpdate(AppState::InGame))
                    .before(set_player_position),
            )
            .add_system(gain_power.in_set(OnUpdate(AppState::InGame)))
            .add_system(set_player_position)
            .add_system(reset_available_powers.in_schedule(OnEnter(AppState::InGame)))
            .add_system(reset_available_powers.in_schedule(OnExit(AppState::InGame)))
            .add_system(
                reset_available_powers
                    .run_if(in_state(AppState::InGame).and_then(on_event::<LevelLoaded>())),
            );
    }
}

#[derive(Component)]
pub struct Player(pub i8, pub i8);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GainPower(pub Power);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UsePower(pub Power, pub Tile);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Power {
    Fertilize,
    Fire,
    Seed,
    Drain,
}

impl ToString for Power {
    fn to_string(&self) -> String {
        match self {
            Power::Fertilize => "Fertilize",
            Power::Fire => "Fire",
            Power::Seed => "Seed",
            Power::Drain => "Drain",
        }
        .to_string()
    }
}

impl Power {
    pub fn ui_class_name(&self) -> String {
        let name = self.to_string();
        format!("card {name}")
    }

    pub fn ui_image(&self) -> String {
        match self {
            Power::Fertilize => "card_fertilize.png",
            Power::Fire => "card_fire.png",
            Power::Seed => "card_seed.png",
            Power::Drain => "card_drain.png",
        }
        .to_string()
    }

    pub fn key_binding(&self) -> String {
        match self {
            Power::Fertilize => "Z",
            Power::Fire => "X",
            Power::Seed => "C",
            Power::Drain => "V",
        }
        .to_string()
    }
}

#[derive(Resource, Clone, Default)]
pub struct AvailablePowers(pub HashMap<Power, usize>);

impl AvailablePowers {
    fn set_value(&mut self, p: Power, v: usize) {
        self.0.insert(p, v);
    }
}

fn reset_available_powers(mut commands: Commands) {
    commands.insert_resource(AvailablePowers::default());
}

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
                    (KeyCode::Z.into(), Action::Fertilize),
                    (KeyCode::C.into(), Action::Seed),
                    (KeyCode::V.into(), Action::Drain),
                    (KeyCode::X.into(), Action::Fire),
                ]),
            },
        ));
    }
}

fn set_player_position(
    mut players: Query<(&Player, &mut Transform), Changed<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut position = None;
    for (p, mut t) in players.iter_mut() {
        t.translation = Vec3::new(p.0 as f32, p.1 as f32, 2.) * TILE_WORLD_SIZE;
        position = Some(t.translation);
    }

    let Some(position) = position else {return;};

    for mut c in camera.iter_mut() {
        c.translation = position + Vec3::Z * 30.;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Fertilize,
    Fire,
    Seed,
    Drain,
}

fn move_player(
    mut player: Query<(&mut Player, &ActionState<Action>)>,
    query: Query<(&Tile, &Ground, &Plant)>,
    mut use_power: EventWriter<UsePower>,
    mut powers: ResMut<AvailablePowers>,
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
            try_use_power(Power::Fertilize, &mut powers, &mut use_power, &p);
        }

        if a.just_pressed(Action::Drain) {
            try_use_power(Power::Drain, &mut powers, &mut use_power, &p);
        }

        if a.just_pressed(Action::Seed) {
            try_use_power(Power::Seed, &mut powers, &mut use_power, &p);
        }

        if a.just_pressed(Action::Fire) {
            try_use_power(Power::Fire, &mut powers, &mut use_power, &p);
        }
    }
}

fn try_use_power(
    power: Power,
    powers: &mut ResMut<AvailablePowers>,
    use_power: &mut EventWriter<UsePower>,
    p: &Mut<Player>,
) {
    let Some(available) = powers.0.get(&power) else {return;};
    let available = *available;
    if available > 0 {
        powers.set_value(power.clone(), available.saturating_sub(1));
        use_power.send(UsePower(power, Tile(p.0, p.1)));
    }
}

fn gain_power(mut gain_power: EventReader<GainPower>, mut powers: ResMut<AvailablePowers>) {
    for GainPower(p) in gain_power.iter() {
        let available = powers.0.get(p).copied().unwrap_or_default();
        powers.set_value(*p, available + 1);
    }
}
