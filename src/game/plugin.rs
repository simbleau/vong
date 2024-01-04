use super::{ball, paddle};
use crate::{
    engine::{camera::CameraTargetTag, physics::PhysicsTag},
    game::{
        ball::BallTag,
        paddle::{PaddleBundle, PaddleProperties},
    },
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vello::{debug::DebugVisualizations, VelloVectorBundle};
use std::f32::consts::PI;

pub struct PongGamePlugin;

impl Plugin for PongGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ball::modify_body_velocity);
        app.add_systems(Update, paddle::move_paddle);
        app.add_systems(Startup, (setup_game, setup_instruction_text));
    }
}

/// Setup the game
pub fn setup_game(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Background
    commands.spawn(VelloVectorBundle {
        vector: asset_server.load("bg.svg"),
        origin: bevy_vello::Origin::Center,
        debug_visualizations: DebugVisualizations::Hidden,
        transform: Transform::from_scale(Vec3::splat(3.0)),
        ..default()
    });

    // Spawn pan
    commands.spawn(VelloVectorBundle {
        vector: asset_server.load("pan.svg"),
        origin: bevy_vello::Origin::Center,
        debug_visualizations: DebugVisualizations::Hidden,
        transform: Transform::from_xyz(-270.0, 343., 0.)
            .with_scale(Vec3::splat(3.3)),
        ..default()
    });
    // Pan physics - Draw oval
    const SEGMENTS: u32 = 100;
    const SCALE_X: f32 = 537.0;
    const SCALE_Y: f32 = 437.0;
    let mut polyline = vec![];
    polyline.push(Vec2::new(SCALE_X, 0.0));
    for r in 1..=SEGMENTS {
        let x = ((r as f32 / SEGMENTS as f32) * 2.0 * PI).cos() * SCALE_X;
        let y = ((r as f32 / SEGMENTS as f32) * 2.0 * PI).sin() * SCALE_Y;
        polyline.push(Vec2::new(x, y));
    }
    commands.spawn(Collider::polyline(polyline, None));

    // Paddle 0
    commands
        .spawn(PaddleBundle {
            sprite: VelloVectorBundle {
                vector: asset_server.load("bacon.svg"),
                origin: bevy_vello::Origin::Center,
                debug_visualizations: DebugVisualizations::Hidden,
                transform: Transform::from_xyz(-385.0, 0.0, 0.0)
                    .with_scale(Vec3::new(-0.2, 0.1, 0.1))
                    .with_rotation(Quat::from_rotation_z(90_f32.to_radians())),
                ..default()
            },
            properties: PaddleProperties {
                player_num: 0,
                ..default()
            },
        })
        .insert(PhysicsTag);

    // Paddle 1
    commands
        .spawn(PaddleBundle {
            sprite: VelloVectorBundle {
                vector: asset_server.load("bacon.svg"),
                origin: bevy_vello::Origin::Center,
                debug_visualizations: DebugVisualizations::Hidden,
                transform: Transform::from_xyz(385.0, 0.0, 0.0)
                    .with_scale(Vec3::new(-0.2, 0.1, 0.1))
                    .with_rotation(Quat::from_rotation_z(90_f32.to_radians())),
                ..default()
            },
            properties: PaddleProperties {
                player_num: 1,
                ..default()
            },
        })
        .insert(PhysicsTag);

    // Egg
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(200.0, 0.0),
            angvel: 0.0,
        })
        .insert(Ccd::enabled())
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Multiply,
        })
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(ColliderMassProperties::MassProperties(MassProperties {
            local_center_of_mass: Vec2::new(20.0, -50.0),
            mass: 1.0,
            principal_inertia: 1000.0,
        }))
        .insert(VelloVectorBundle {
            vector: asset_server.load("egg.svg"),
            origin: bevy_vello::Origin::Center,
            debug_visualizations: DebugVisualizations::Hidden,
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..Default::default()
        })
        .insert(CameraTargetTag)
        .insert(BallTag)
        .insert(PhysicsTag);
}

fn setup_instruction_text(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_sections([TextSection::new(
            "Instructions",
            TextStyle {
                font_size: 22.0,
                color: Color::WHITE,
                ..default()
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

    let instructions = [
        "Camera Zoom: `Up` and `Down`",
        "Move Players: `W`, `S`, `I` and `K`",
        "Move Camera: `PgUp`, `PgDown`, `Home`, `Delete`",
        "Follow Egg: `C`",
        "Debug Visualizations: `Q`",
    ];
    for (i, instruction) in instructions.iter().enumerate() {
        commands.spawn(
            TextBundle::from_sections([TextSection::new(
                *instruction,
                TextStyle {
                    font_size: 22.0,
                    color: Color::WHITE,
                    ..default()
                },
            )])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(i as f32 * 22.0 + 42.0),
                left: Val::Px(10.0),
                flex_wrap: FlexWrap::NoWrap,
                ..default()
            }),
        );
    }
}
