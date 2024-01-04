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
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vong".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::PRIMARY),
                        ..default()
                    }
                    .into(),
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
