use bevy::prelude::*;
use bevy_vello::VelloVectorBundle;

#[derive(Component)]
pub struct PaddleProperties {
    pub speed: f32,
    pub player_num: usize,
    pub direction: f32,
}

impl Default for PaddleProperties {
    fn default() -> Self {
        Self {
            speed: 24.0 * 15.0,
            player_num: 0,
            direction: 0.0,
        }
    }
}

#[derive(Default, Bundle)]
pub struct PaddleBundle {
    pub properties: PaddleProperties,
    pub sprite: VelloVectorBundle,
}

pub fn move_paddle(
    mut query: Query<(&mut Transform, &mut PaddleProperties)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut paddle_transform, mut paddle) in query.iter_mut() {
        let mut direction = 0.0;

        if paddle.player_num == 0 {
            if keyboard_input.pressed(KeyCode::W) {
                direction += 1.0;
            }

            if keyboard_input.pressed(KeyCode::S) {
                direction -= 1.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::I) {
                direction += 1.0;
            }

            if keyboard_input.pressed(KeyCode::K) {
                direction -= 1.0;
            }
        }

        paddle.direction = direction;

        paddle_transform.translation.y +=
            direction * paddle.speed * time.delta_seconds();
        paddle_transform.translation.y =
            paddle_transform.translation.y.clamp(-225.0, 175.0);
    }
}
