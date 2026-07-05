use avian3d::dynamics::rigid_body::RigidBodyDisabled;
use bevy::{ecs::{bundle::Bundle, component::Component, hierarchy::ChildOf}, transform::components::Transform};

#[derive(Component)]
#[require(Transform = Transform::default())]
pub struct Player;

#[derive(Component)]
#[require(Transform = Transform::default())]
pub struct PlayerHead;

#[derive(Component, Default)]
pub struct Grabbed;

#[derive(Bundle)]
pub struct GrabbedBundle
{
    pub grabbed: Grabbed,
    pub child_of: ChildOf,
    pub rigidbody_disabled: RigidBodyDisabled,
}
