use bevy::prelude::*;

const OBS_SPEED: f32 = 140.;
const PIPE_HEIGHT: f32 = 256.;
const HOLE_HEIGHT_RANGE: f32 = 400.;
const GAP: f32 = 50.;

use crate::SCALE;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TriggerSpawnState>()
            .add_systems(Update, (event_trigger, move_obstacle));
    }
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct TriggerSpawnState {
    event_timer: Timer,
}

impl Default for TriggerSpawnState {
    fn default() -> Self {
        TriggerSpawnState {
            event_timer: Timer::from_seconds(4., TimerMode::Repeating),
        }
    }
}

fn event_trigger(
    time: Res<Time>,
    mut state: ResMut<TriggerSpawnState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !state.event_timer.tick(time.delta()).finished() {
        return;
    }

    let pipe_gap = (PIPE_HEIGHT * SCALE) / 2. + GAP;
    let h = (rand::random::<f32>() - 0.5) * HOLE_HEIGHT_RANGE;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pipo.png"),
            transform: Transform::from_translation(Vec3::new(700., h + pipe_gap, 0.))
                .with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Obstacle,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pipo.png"),
            transform: Transform::from_translation(Vec3::new(700., h - pipe_gap, 0.))
                .with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Obstacle,
    ));
}

fn move_obstacle(time: Res<Time>, mut query: Query<(&mut Transform, &Obstacle)>) {
    for mut i in query.iter_mut() {
        i.0.translation.x -= time.delta_seconds_f64() as f32 * OBS_SPEED;
    }
}
