use bevy::prelude::*;

const MOVE_SPEED: f32 = 0.4;

#[derive(Component)]
struct Moveable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, init)
        .add_systems(Update, move_entity)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Draw
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(30., 30.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        },
        Moveable,
    ));
}

fn move_entity(
    mut q: Query<&mut Transform, With<Moveable>>,
    t: Res<Time>,
    key: Res<Input<KeyCode>>,
) {
    for mut e in q.iter_mut() {
        if key.pressed(KeyCode::W) {
            e.translation.y += t.delta().as_millis() as f32 * MOVE_SPEED;
        }
        if key.pressed(KeyCode::A) {
            e.translation.x -= t.delta().as_millis() as f32 * MOVE_SPEED;
        }
        if key.pressed(KeyCode::S) {
            e.translation.y -= t.delta().as_millis() as f32 * MOVE_SPEED;
        }
        if key.pressed(KeyCode::D) {
            e.translation.x += t.delta().as_millis() as f32 * MOVE_SPEED;
        }
    }
}
