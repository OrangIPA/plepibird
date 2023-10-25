use bevy::prelude::*;

use crate::{SCALE, WIN_HEIGHT, WIN_WIDTH};

const PRESSED_VELOCITY: f32 = 300.;
const ACCELERATION_CONSTANT: f32 = 0.001;
const MAX_VELOCITY: f32 = 500.;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .add_systems(Update, move_entity);
    }
}

enum FlapDirection {
    Vertical,
    Horizontal,
}

#[derive(Component)]
struct Bird {
    v_vel: f32,
    h_vel: f32,
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
            v_vel: 0.,
            h_vel: 0.,
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

    if key.just_pressed(KeyCode::W)
        || (mouse.just_pressed(MouseButton::Left) && !mouse.just_pressed(MouseButton::Right))
    {
        berd.1.v_vel = PRESSED_VELOCITY;
        berd.1.flap = FlapDirection::Vertical;
    }

    if key.just_pressed(KeyCode::D)
        || (mouse.just_pressed(MouseButton::Right) && !mouse.just_pressed(MouseButton::Left))
    {
        berd.1.h_vel = PRESSED_VELOCITY;
        berd.1.flap = FlapDirection::Horizontal;
    }

    match berd.1.flap {
        FlapDirection::Horizontal => {
            let move_x = berd.1.h_vel * t.delta().as_millis() as f32 * ACCELERATION_CONSTANT;
            let accl_x = (t.delta().as_millis() as f32).min(MAX_VELOCITY);
            berd.1.h_vel -= accl_x / 2.;
            berd.0.translation.x = (berd.0.translation.x + move_x)
                .min(WIN_WIDTH / 2.)
                .max(-WIN_WIDTH / 2.);
            berd.1.h_vel -= accl_x / 2.;
        }
        FlapDirection::Vertical => {
            let move_y = berd.1.v_vel * t.delta().as_millis() as f32 * ACCELERATION_CONSTANT;
            let accl_y = (t.delta().as_millis() as f32).min(MAX_VELOCITY);
            berd.1.v_vel -= accl_y / 2.;
            berd.0.translation.y = (berd.0.translation.y + move_y)
                .max(-WIN_HEIGHT / 2.)
                .min(WIN_HEIGHT / 2.);
            berd.1.v_vel -= accl_y / 2.;
        }
    }
}
