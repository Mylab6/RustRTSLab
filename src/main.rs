use bevy::prelude::*;
use rand::Rng;

struct Cube {
    target: Vec3,
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_cubes)
        .add_system(update_targets)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn cubes
    for _ in 0..10 {
        let target = random_target_position();
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(random_start_position()),
            ..default()
        })
        .insert(Cube { target, speed: 2.0 });
    }
}

fn random_start_position() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(-5.0..5.0),
        rng.gen_range(-5.0..5.0),
        rng.gen_range(-5.0..5.0),
    )
}

fn random_target_position() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(-5.0..5.0),
        rng.gen_range(-5.0..5.0),
        rng.gen_range(-5.0..5.0),
    )
}

fn move_cubes(mut query: Query<(&Cube, &mut Transform)>, time: Res<Time>) {
    for (cube, mut transform) in query.iter_mut() {
        let direction = (cube.target - transform.translation).normalize();
        transform.translation += direction * cube.speed * time.delta_seconds();
    }
}

fn update_targets(mut query: Query<(&mut Cube, &Transform)>) {
    for (mut cube, transform) in query.iter_mut() {
        if transform.translation.distance(cube.target) < 0.1 {
            cube.target = random_target_position();
        }
    }
}
