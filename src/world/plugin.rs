use avian3d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Box;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_world);
    }
}

fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(Plane3d::new(Vec3::Y, Vec2::splat(300.0)));
    let mesh_handle = meshes.add(mesh.clone());
    let material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });

    commands.spawn((
        MeshMaterial3d(material.clone()),
        Mesh3d(mesh_handle),
        Collider::convex_hull_from_mesh(&mesh).unwrap(),
        RigidBody::Static,
        Transform::from_translation(-Vec3::Y * 20.0),
    ));

    let cube_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 0.0),
        ..Default::default()
    });

    let cubes = [
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(10.0, 10.0, 10.0),
        Vec3::new(5.0, 10.0, 5.0),
        Vec3::new(5.0, 5.0, 10.0),
    ]
    .map(|cuboid| Mesh::from(Cuboid::from_size(cuboid)));

    let cube_mesh: Vec<_> = cubes.iter().map(|cube| meshes.add(cube.clone())).collect();

    for i in 1..300 {
        let cube_handle = cube_mesh[i % cube_mesh.len()].clone();
        let cube = &cubes[i % cubes.len()];

        let position = Vec3::new(
            rand::random_range(-80.0..80.0),
            rand::random_range(30.0..100.0),
            rand::random_range(-80.0..80.0),
        );

        commands.spawn((
            MeshMaterial3d(cube_material.clone()),
            Mesh3d(cube_handle),
            Collider::convex_hull_from_mesh(&cube).unwrap(),
            RigidBody::Dynamic,
            Box,
            Transform::from_translation(position),
        ));
    }
}
