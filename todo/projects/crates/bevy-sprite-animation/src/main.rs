use bevy::{prelude::*, render::camera::OrthographicProjection};
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate.system())
        .run();
}

/// Set up a 2D camera view and a simple white sprite
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let material_handle = materials.add(Color::WHITE.into());
    let sprite_bundle = SpriteBundle {
        material: material_handle,
        sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        ..Default::default()
    };
    commands.spawn_bundle(sprite_bundle);
}

/// Animate a sprite's rotation and scale
fn animate(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        let sec = time.seconds_since_startup() as f32;
        let scale = (0.5 * ((secs * 0.5).sin() + 1.0)).max(0.1);
        let rotation = secs * 2.0;
        sprite.scale = Vec2::new(scale, scale);
        sprite.rotate(rotation);
    }
}
