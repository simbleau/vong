use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct BallTag;

/* Set the velocities inside of a system. */
pub fn modify_body_velocity(
    mut velocities: Query<&mut Velocity, With<BallTag>>,
) {
    for mut vel in velocities.iter_mut() {
        // Get a little faster gradually until terminal
        vel.linvel *= 1.01;
        // Clamp
        vel.linvel = vel
            .linvel
            .clamp(Vec2::new(-400.0, -400.0), Vec2::new(400., 400.));
        debug!(
            linvel = format!(
                "{:.2}",
                (vel.linvel.x.abs() + vel.linvel.y.abs()).sqrt()
            ),
            angvel = vel.angvel
        );
    }
}
