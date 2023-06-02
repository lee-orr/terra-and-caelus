use std::{f32::consts::PI, ops::Mul};

use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};

use crate::{colors, states::AppState};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_loading.in_set(OnUpdate(AppState::LoadingAssets)));
    }
}

#[derive(Component)]
struct LoadingEntity;

const LOADING_ANIM_SPEED: f32 = 5.;
const LOADING_ANIM_SIZE: f32 = 50.0;

fn draw_loading(mut painter: ShapePainter, time: Res<Time>) {
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
