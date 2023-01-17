use std::f32::consts::PI;

use crate::prelude::*;

pub fn turret(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<(&ActionState<Action>, &Transform), With<Turret>>,
) {
    for (input, transform) in query.iter() {
        if input.just_pressed(Action::Turret) {
            let how_many = 6.0;
            let angle = PI / 16.0;
            let step_size = (2.0 * angle) / how_many;

            let mut curr = -angle;
            while curr < angle {
                let mut euler = transform.rotation.to_euler(EulerRot::XYZ);
                euler.2 += curr;
                let quat = Quat::from_euler(EulerRot::XYZ, euler.0, euler.1, euler.2);
                spawn_projectile(&mut commands, transform.translation, quat, time.clone());
                curr += step_size;
            }
        }
    }
}
