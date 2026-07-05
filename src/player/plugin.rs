use avian3d::{
    collision::collider::Collider,
    dynamics::{
        joints::FixedJoint,
        rigid_body::{
            AngularVelocity, LinearVelocity, RigidBody, RigidBodyDisabled,
            mass_properties::bevy_heavy::ComputeMassProperties3d,
        },
    },
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    player::components::{Grabbed, GrabbedBundle, Player, PlayerHead},
    world::plugin::Box,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (create_player, attach_camera_to_player).chain());
        app.add_systems(PreUpdate, (player_control, grab_raycast));
    }
}

fn create_player(mut commands: Commands) {
    commands.spawn((
        Player,
        RigidBody::Kinematic,
        children![(PlayerHead, Transform::from_translation(Vec3::Y),)],
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
    mut player_transform: Single<&mut Transform, (With<Player>, Without<PlayerHead>)>,
    mut player_head_transform: Single<&mut Transform, (With<PlayerHead>, Without<Player>)>,
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

    player_transform.translation += dir / 3.0;

    let sum = mouse_movement.read().map(|data| data.delta).sum::<Vec2>();

    player_transform.rotate_local_y(-sum.x / 200.0);

    let mut head_rotation: Vec3 = player_head_transform
        .rotation
        .to_euler(EulerRot::XYX)
        .into();

    head_rotation.x =
        (head_rotation.x + (-sum.y / 500.0)).clamp(-90.0_f32.to_radians(), 60.0_f32.to_radians());

    player_head_transform.rotation = Quat::from_euler(
        EulerRot::XYX,
        head_rotation.x,
        head_rotation.y,
        head_rotation.z,
    );
}

fn grab_raycast(
    player: Single<(Entity, &GlobalTransform), With<PlayerHead>>,
    grabbed: Option<Single<(Entity, &GlobalTransform), (With<Grabbed>, With<RigidBodyDisabled>)>>,
    spatial_query: SpatialQuery,
    mut boxes: Query<
        (
            Entity,
            &GlobalTransform,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        With<Box>,
    >,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(grabbed) = grabbed {
            let reparented = grabbed.1.reparented_to(&GlobalTransform::IDENTITY);

            commands.entity(grabbed.0).remove::<GrabbedBundle>();
            commands.entity(grabbed.0).insert(reparented);
        } else {
            let filter = SpatialQueryFilter::default();

            let origin = player.1.translation();
            let direction = player.1.forward();

            if let Some(hit) = spatial_query.cast_ray(origin, direction, 1000.0, false, &filter)
                && let Ok(mut box_entity) = boxes.get_mut(hit.entity)
            {
                let reparented = box_entity.1.reparented_to(player.1);

                *box_entity.2 = LinearVelocity::ZERO;
                *box_entity.3 = AngularVelocity::ZERO;

                commands.entity(box_entity.0).insert((
                    GrabbedBundle {
                        child_of: ChildOf(player.0),
                        grabbed: Grabbed,
                        rigidbody_disabled: RigidBodyDisabled,
                    },
                    reparented,
                ));
            }
        }
    }
}
