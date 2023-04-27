use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn main() {
 App::new()
  .add_plugins(DefaultPlugins)
  .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
  .add_plugin(RapierDebugRenderPlugin::default())
  .add_startup_system(setup_graphics)
  .add_startup_system(setup_physics)
  .add_system(print_ball_altitude)
  .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(100.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
    });
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
