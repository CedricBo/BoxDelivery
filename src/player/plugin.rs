use std::ops::Add;

use bevy::{input::mouse::MouseMotion, math::VectorSpace, prelude::*};

use crate::player::components::{Player, PlayerHead};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (create_player, attach_camera_to_player).chain());
        app.add_systems(Update, player_control);
    }
}

fn create_player(mut commands: Commands) {
    commands.spawn((
        Player,
        children![(PlayerHead, Transform::from_translation(Vec3::Y))],
    ));
}

fn attach_camera_to_player(
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
    player: Single<Entity, With<PlayerHead>>,
) {
    let mut camera_commands = commands.entity(camera.entity());

    camera_commands.insert(ChildOf(player.entity()));
}

fn player_control(
    mut player_transform: Single<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_movement: MessageReader<MouseMotion>,
) {
    let dir = keyboard.get_pressed().fold(Vec3::ZERO, |dir, code| {
        dir + match code {
            KeyCode::KeyW => player_transform.forward().xyz(),
            KeyCode::KeyS => player_transform.back().xyz(),
            KeyCode::KeyA => player_transform.left().xyz(),
            KeyCode::KeyD => player_transform.right().xyz(),
            _ => Vec3::ZERO,
        }
    });

    player_transform.translation += dir;

    let sum = mouse_movement.read().map(|data| data.delta).sum::<Vec2>();

    player_transform.rotate_local_y(-sum.x / 100.0);
}
