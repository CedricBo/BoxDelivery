use avian3d::{PhysicsPlugins, debug_render::PhysicsDebugPlugin};
use bevy::DefaultPlugins;

use crate::{camera::CameraPlugin, player::plugin::PlayerPlugin, world::plugin::WorldPlugin};

mod camera;
mod player;
mod world;

fn main() {
    let mut app = bevy::app::App::new();

    app.add_plugins((
        DefaultPlugins,
        CameraPlugin,
        PlayerPlugin,
        WorldPlugin,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin
    ));

    app.run();
}
