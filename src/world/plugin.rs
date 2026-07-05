use avian3d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

pub struct WorldPlugin;

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
    let mesh = Mesh::from(Plane3d::new(Vec3::Y, Vec2::splat(100.0)));
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
        Transform::from_translation(-Vec3::Y * 20.0)
    ));

    let cube = Mesh::from(Cuboid::new(10.0, 10.0, 10.0));
    let cube_handle = meshes.add(cube.clone());
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn((
        MeshMaterial3d(cube_material),
        Mesh3d(cube_handle),
        Collider::convex_hull_from_mesh(&cube).unwrap(),
        RigidBody::Dynamic,
        Transform::from_translation((0.0, 50.0, -50.0).into()),
    ));
}
