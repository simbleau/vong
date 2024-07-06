#![allow(clippy::type_complexity)]

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    window::PresentMode,
};
use bevy_vello::VelloPlugin;
use engine::{camera::CameraPlugin, physics::PhysicsPlugin, ui::UiPlugin};
use game::PongGamePlugin;
mod engine;
mod game;

pub fn main() {
    // Check for WebGPU
    if !webgpu_check::is_webgpu_available() {
        const WEBGPU_FORM: &str = include_str!("../assets/webgpu_form.html");
        replace_html::replace_by_id("canvas-container", WEBGPU_FORM).unwrap();
        return;
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vong".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        canvas: Some("#c".to_string()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::PRIMARY),
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins(VelloPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PongGamePlugin)
        .run();
}
