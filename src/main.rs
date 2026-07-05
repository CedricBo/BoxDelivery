use avian3d::{PhysicsPlugins, debug_render::PhysicsDebugPlugin};
use bevy::{
    DefaultPlugins,
    app::{Startup, Update},
    ecs::{
        query::With,
        system::{Local, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    render::view::window,
    window::{CursorOptions, PrimaryWindow, Window},
};

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
        PhysicsDebugPlugin,
    ));

    app.add_systems(Update, lock_cursor);

    app.run();
}

fn lock_cursor(
    mut cursor: Single<&mut CursorOptions>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut previous_focus: Local<bool>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = bevy::window::CursorGrabMode::None;
        cursor.visible = true;
    }

    if window.focused && !*previous_focus {
        cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        cursor.visible = false;
    }

    *previous_focus = window.focused;
}
