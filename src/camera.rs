use bevy::{app::{Plugin, Startup}, camera::Camera3d, ecs::system::Commands};

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands)
{
    commands.spawn(Camera3d::default());
}