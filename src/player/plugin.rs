use avian3d::{
    dynamics::{
        joints::FixedJoint, rigid_body::{RigidBody, sleeping::SleepingDisabled},
    }, physics_transform::{Position, Rotation}, spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    player::components::{Grabbed, Player, PlayerGrab, PlayerHead},
    world::plugin::Box,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, create_player);
        app.add_systems(Update, grab_raycast);

        app.add_systems(
            FixedUpdate,
            (
                (player_control)/* .before(TransformSystems::Propagate) */,
                camera_follow_player/* .after(TransformSystems::Propagate) */,
            ),
        );
    }
}

fn create_player(mut commands: Commands) {
    commands.spawn((
        Player,
        RigidBody::Kinematic,
        SleepingDisabled,
        children![
            (PlayerHead, Transform::from_translation(Vec3::Y),),
            (
                PlayerGrab,
                Transform::from_translation(Vec3::new(10.0, -6.0, -20.0))
            )
        ],
    ));
}

fn camera_follow_player(
    mut camera: Single<&mut Transform, With<Camera3d>>,
    player: Single<&GlobalTransform, With<PlayerHead>>,
) {
    camera.translation = player.translation();
    camera.rotation = player.rotation();
}

fn player_control(
    mut player_transform: Single<
        (&mut Transform, &mut Position, &mut Rotation),
        (With<Player>, Without<PlayerHead>),
    >,
    mut player_head_transform: Single<&mut Transform, (With<PlayerHead>, Without<Player>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_movement: MessageReader<MouseMotion>,
) {
    let dir = keyboard.get_pressed().fold(Vec3::ZERO, |dir, code| {
        dir + match code {
            KeyCode::KeyW => player_transform.0.forward().xyz(),
            KeyCode::KeyS => player_transform.0.back().xyz(),
            KeyCode::KeyA => player_transform.0.left().xyz(),
            KeyCode::KeyD => player_transform.0.right().xyz(),
            _ => dir,
        }
    });

    if dir != Vec3::ZERO
    {
        println!("Dir: {:?}", dir);

        player_transform.0.translation += dir.normalize() / 3.0;
    }

    let sum = mouse_movement.read().map(|data| data.delta).sum::<Vec2>() / 5.0;

    // player_transform.0.rotate_local_y();
    if sum != Vec2::ZERO {
        player_transform.2.0 *= Quat::from_rotation_y(-sum.x.to_radians());
        // player_transform.2.0 = player_transform.2.0.normalize();

        let mut head_rotation: Vec3 = player_head_transform
            .rotation
            .to_euler(EulerRot::XYX)
            .into();

        head_rotation.x = (head_rotation.x + (-sum.y / 100.0))
            .clamp(-90.0_f32.to_radians(), 60.0_f32.to_radians());

        player_head_transform.rotation = Quat::from_euler(
            EulerRot::XYX,
            head_rotation.x,
            head_rotation.y,
            head_rotation.z,
        );
    }
}

fn grab_raycast(
    player_head: Single<
        (Entity, &GlobalTransform),
        (With<PlayerHead>, Without<Player>, Without<PlayerGrab>),
    >,
    player: Single<
        (Entity, &GlobalTransform),
        (With<Player>, Without<PlayerHead>, Without<PlayerGrab>),
    >,
    boxes: Query<Entity, With<Box>>,
    player_grab: Single<&GlobalTransform, (With<PlayerGrab>, Without<PlayerHead>, Without<Player>)>,
    grabbed: Option<Single<(Entity, &GlobalTransform), With<Grabbed>>>,
    spatial_query: SpatialQuery,
    joint: Option<Single<Entity, With<FixedJoint>>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(grabbed) = grabbed {

            commands.entity(grabbed.0).remove::<Grabbed>();
            commands.entity(joint.unwrap().into_inner()).despawn();
        } else {
            let filter = SpatialQueryFilter::default();

            let origin = player_head.1.translation();
            let direction = player_head.1.forward();

            if let Some(hit) = spatial_query.cast_ray(origin, direction, 1000.0, false, &filter)
                && boxes.contains(hit.entity)
            {
                let grab_reparented = player_grab.reparented_to(player.1);

                commands.spawn(
                    FixedJoint::new(player.0, hit.entity)
                        .with_local_anchor1(grab_reparented.translation)
                        .with_local_anchor2(Vec3::ZERO)
                        .with_point_compliance(0.0),
                );

                commands.entity(hit.entity).insert(Grabbed);
            }
        }
    }
}
