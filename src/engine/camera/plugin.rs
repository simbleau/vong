use std::ops::DerefMut;

use bevy::prelude::*;
use dolly::{glam::Vec3Swizzles, prelude::*};

#[derive(Default)]
pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraTargetTag;

const PAN_SPEED: f32 = 256.0;
const MIN_ZOOM: f32 = -0.99;
const MAX_ZOOM: f32 = 3.0;
const ZOOM_SPEED: f32 = 2.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, drive_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_dolly_cam() -> CameraRig {
    CameraRig::builder()
        .with(Position::new(dolly::glam::Vec3::new(0.0, 0.0, 1.0)))
        .with(Arm::new(dolly::glam::Vec3::new(0.0, 0.0, 1.0)))
        .with(Smooth::new_position(1.5).predictive(false))
        .build()
}

fn drive_camera(
    time: Res<Time>,
    mut proj_query: Query<&mut OrthographicProjection>,
    query: Query<&Transform, With<CameraTargetTag>>,
    mut query_cam: Query<
        &mut Transform,
        (With<Camera>, Without<CameraTargetTag>),
    >,
    mut camera_rig_res: Local<Option<CameraRig>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut flag: Local<bool>,
) {
    let mut camera_rig = if let Some(cam) = camera_rig_res.deref_mut().take() {
        cam
    } else {
        init_dolly_cam()
    };

    let mut proj = proj_query.single_mut();
    if keyboard_input.pressed(KeyCode::Up) {
        camera_rig.driver_mut::<Arm>().offset.z -=
            proj.scale * ZOOM_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Down) {
        camera_rig.driver_mut::<Arm>().offset.z +=
            proj.scale * ZOOM_SPEED * time.delta_seconds();
    }
    camera_rig.driver_mut::<Arm>().offset.z = camera_rig
        .driver_mut::<Arm>()
        .offset
        .z
        .clamp(MIN_ZOOM, MAX_ZOOM);

    if keyboard_input.just_pressed(KeyCode::C) {
        *flag = !*flag;
    }

    if *flag {
        if let Some(target_transform) = query.iter().next() {
            let translation_decomp: [f32; 3] =
                target_transform.translation.into();
            camera_rig.driver_mut::<Position>().position.x =
                translation_decomp[0];
            camera_rig.driver_mut::<Position>().position.y =
                translation_decomp[1];
        }
    } else {
        let mut pos = camera_rig.driver_mut::<Position>().position;

        if keyboard_input.pressed(KeyCode::PageDown) {
            pos.x += PAN_SPEED * time.delta_seconds() * proj.scale;
        }

        if keyboard_input.pressed(KeyCode::Delete) {
            pos.x -= PAN_SPEED * time.delta_seconds() * proj.scale;
        }

        if keyboard_input.pressed(KeyCode::Home) {
            pos.y += PAN_SPEED * time.delta_seconds() * proj.scale;
        }

        if keyboard_input.pressed(KeyCode::End) {
            pos.y -= PAN_SPEED * time.delta_seconds() * proj.scale;
        }

        camera_rig.driver_mut::<Position>().position = pos;
    }

    camera_rig.update(time.delta_seconds());

    if let Some(mut camera_transform) = query_cam.iter_mut().next() {
        let translation_decomp: [f32; 2] =
            camera_rig.final_transform.position.xy().into();
        let rotation_decomp: [f32; 4] =
            camera_rig.final_transform.rotation.into();

        camera_transform.translation.x = translation_decomp[0];
        camera_transform.translation.y = translation_decomp[1];
        camera_transform.rotation = Quat::from_array(rotation_decomp);
        proj.scale = camera_rig.final_transform.position.z;
    }

    *camera_rig_res = Some(camera_rig);
}
