use std::f32::consts::PI;

use crate::prelude::*;

pub fn turret(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&ActionState<Action>, &Transform, &mut ExternalImpulse), With<Turret>>,
) {
    for (input, transform, mut external_force) in query.iter_mut() {
        if input.just_pressed(Action::Turret) {
            let how_many = 4.0;
            let angle = PI / 8.0;
            let step_size = (2.0 * angle) / how_many;

            let mut curr = -angle - step_size;
            while curr < angle {
                curr += step_size;
                let mut euler = transform.rotation.to_euler(EulerRot::XYZ);
                euler.2 += curr;
                let quat = Quat::from_euler(EulerRot::XYZ, euler.0, euler.1, euler.2);
                spawn_projectile(&mut commands, transform.translation, quat, time.clone());

                // Add impulse in the opposite direction of our transform
                let direction = -(transform.rotation * Vec3::Y).normalize();
                external_force.impulse = Vec2::new(direction.x, direction.y) * PROJECTILE_KNOCKBACK;
            }
        }
    }
}
