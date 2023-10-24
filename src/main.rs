use bevy::prelude::*;

mod bird;
mod obstacle;

pub const SCALE: f32 = 2.;
pub const WIN_WIDTH: f32 = 1200.;
pub const WIN_HEIGHT: f32 = 700.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Flappy Bird".into(),
                        resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(bird::BirdPlugin)
        .add_plugins(obstacle::ObstaclePlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
