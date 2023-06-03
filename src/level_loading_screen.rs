use std::{f32::consts::PI, ops::Mul};

use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};

use crate::{
    colors,
    level_asset::{CurrentLevel, LevelAsset},
    states::AppState,
};

pub struct LevelLoadingScreenPlugin;

impl Plugin for LevelLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_loading.in_set(OnUpdate(AppState::LoadingLevel)));
    }
}

#[derive(Component)]
struct LoadingEntity;

const LOADING_ANIM_SPEED: f32 = 5.;
const LOADING_ANIM_SIZE: f32 = 50.0;

fn draw_loading(
    mut painter: ShapePainter,
    time: Res<Time>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<LevelAsset>>,
    mut commands: Commands,
) {
    let Some(current_level) = current_level.0.as_ref() else {
        commands.insert_resource(NextState(Some(AppState::Menu)));
        return;
    };
    if level_assets.get(current_level).is_some() {
        commands.insert_resource(NextState(Some(AppState::InGame)));
        return;
    }
    let offset = time.elapsed_seconds().mul(LOADING_ANIM_SPEED);
    let offset_x = offset.sin();
    let offset_y = offset.cos();
    let location = Vec3::new(offset_x, offset_y, 0.) * LOADING_ANIM_SIZE;

    painter.set_translation(location);
    painter.color = colors::LIGHT;
    painter.circle(15.);

    let offset = offset + 2. * PI / 3.;
    let offset_x = offset.sin();
    let offset_y = offset.cos();
    let location = Vec3::new(offset_x, offset_y, 0.) * LOADING_ANIM_SIZE;

    painter.set_translation(location);
    painter.color = colors::PALE;
    painter.circle(15.);

    let offset = offset + 2. * PI / 3.;
    let offset_x = offset.sin();
    let offset_y = offset.cos();
    let location = Vec3::new(offset_x, offset_y, 0.) * LOADING_ANIM_SIZE;

    painter.set_translation(location);
    painter.color = colors::DARK;
    painter.circle(15.);
}
