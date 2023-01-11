use crate::prelude::*;

pub fn modifier_linear_speed(mut query: Query<(&mut BaseStats, &LinearSpeedModifier)>) {
    query
        .iter_mut()
        .for_each(|(mut base_stat, linear_speed_modifier)| {
            base_stat.linear_speed = base_stat.linear_speed * (*linear_speed_modifier).0;
        });
}

pub fn modifier_angular_speed(mut query: Query<(&mut BaseStats, &AngularSpeedModifier)>) {
    query
        .iter_mut()
        .for_each(|(mut base_stat, angular_speed_modifier)| {
            base_stat.angular_speed = base_stat.angular_speed * (*angular_speed_modifier).0;
        });
}

pub fn modifier_angular_degrade(mut query: Query<(&mut BaseStats, &AngularDegradeModifier)>) {
    query
        .iter_mut()
        .for_each(|(mut base_stat, angular_degrade_modifier)| {
            base_stat.angular_degrade *= (*angular_degrade_modifier).0;
        });
}
