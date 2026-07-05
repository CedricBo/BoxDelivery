use bevy::{ecs::component::Component, transform::components::Transform};

#[derive(Component)]
#[require(Transform = Transform::default())]
pub struct Player;

#[derive(Component)]
#[require(Transform = Transform::default())]
pub struct PlayerHead;
