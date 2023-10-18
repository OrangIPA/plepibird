use bevy::prelude::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TriggerSpawnState>()
            .add_systems(Update, event_trigger);
    }
}

struct Obstacle;

#[derive(Resource)]
struct TriggerSpawnState {
    event_timer: Timer,
}

impl Default for TriggerSpawnState {
    fn default() -> Self {
        TriggerSpawnState {
            event_timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

fn event_trigger(time: Res<Time>, mut state: ResMut<TriggerSpawnState>, mut commands: Commands) {
    if !state.event_timer.tick(time.delta()).finished() {return;}
    // commands.spawn(bundle)
}
