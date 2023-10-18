mod bird;
mod obstacle;

pub const SCALE: f32 = 2.;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bird::BirdPlugin)
        .add_plugins(obstacle::ObstaclePlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}