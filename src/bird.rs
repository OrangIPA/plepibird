use bevy::prelude::*;
use core::ops;

use crate::{SCALE, WIN_HEIGHT, WIN_WIDTH};

const PRESSED_VELOCITY: f32 = 400.;
const VELOCITY_CONSTANT: f32 = 0.001;
const MAX_VELOCITY: f32 = 700.;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .add_systems(Update, move_entity);
    }
}

#[derive(Clone)]
enum FlapDirection {
    Vertical,
    Horizontal,
}

impl ops::Not for FlapDirection {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            FlapDirection::Horizontal => FlapDirection::Vertical,
            FlapDirection::Vertical => FlapDirection::Horizontal,
        }
    }
}

#[derive(Component)]
struct Bird {
    vel: f32,
    flap: FlapDirection,
}

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Bird
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("berd.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(SCALE, SCALE, SCALE)),
            ..Default::default()
        },
        Bird {
            vel: 0.,
            flap: FlapDirection::Vertical,
        },
    ));
}

fn move_entity(
    mut q: Query<(&mut Transform, &mut Bird)>,
    t: Res<Time>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut berd = q.single_mut();

    if key.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        berd.1.vel = PRESSED_VELOCITY;
        berd.1.flap = !berd.1.flap.clone();
    }

    match berd.1.flap {
        FlapDirection::Horizontal => {
            let move_x = berd.1.vel * t.delta().as_millis() as f32 * VELOCITY_CONSTANT;
            let accl_x = (t.delta().as_millis() as f32).min(MAX_VELOCITY);
            berd.1.vel -= accl_x / 2.;
            berd.0.translation.x = (berd.0.translation.x + move_x)
                .min(WIN_WIDTH / 2.)
                .max(-WIN_WIDTH / 2.);
            berd.1.vel -= accl_x / 2.;
        }
        FlapDirection::Vertical => {
            let move_y = berd.1.vel * t.delta().as_millis() as f32 * VELOCITY_CONSTANT;
            let accl_y = (t.delta().as_millis() as f32).min(MAX_VELOCITY);
            berd.1.vel -= accl_y / 2.;
            berd.0.translation.y = (berd.0.translation.y + move_y)
                .max(-WIN_HEIGHT / 2.)
                .min(WIN_HEIGHT / 2.);
            berd.1.vel -= accl_y / 2.;
        }
    }
}
