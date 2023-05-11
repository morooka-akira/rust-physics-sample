use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        // NOTE: add_pluginsはプラグインの追加
        // DefaultPluginsには、画面描画を含む基本機能が含まれている
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        // NOTE: add_startup_systemは起動時に1回だけ呼ばれる
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        // NOTE: add_systemsは実行中のシステム関数を追加
        .add_systems((circle_move_left_right,))
        .add_systems((box_movement_system,))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Default, Component)]
struct CircleTag;

#[derive(Default, Component)]
struct CircleSpeed {
    speed: f32,
}

#[derive(Default, Component)]
struct BoxTag;

fn setup_graphics(mut commands: Commands) {
    // 2dのカメラフレームをセットする
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        CircleTag::default(),
        CircleSpeed { speed: 3. },
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(50., 50., 0.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
            ..default()
        },
        BoxTag::default(),
    ));
}

// 左右に動く
fn circle_move_left_right(mut query: Query<(&mut CircleSpeed, &mut Transform)>) {
    let (mut speed, mut transform) = query.single_mut();

    if transform.translation.x > 300. || transform.translation.x < -300. {
        speed.speed = 0. - speed.speed;
    }
    transform.translation.x += speed.speed;
}

fn box_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&BoxTag, &mut Transform)>,
) {
    let (_, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        println!("left:")
    }

    if keyboard_input.pressed(KeyCode::Right) {
        println!("right:");
        let movement_direction = transform.rotation * Vec3::X;
        transform.translation += movement_direction * 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        println!("up:")
    }
}
