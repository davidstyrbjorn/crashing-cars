use crate::prelude::*;

pub fn goal_keeper(time: Res<Time>, mut query: Query<&mut Transform, With<GoalKeeper>>) {
    query.iter_mut().for_each(|mut transform| {
        transform.translation.y = time.elapsed_seconds().sin() * 100.0;
    });
}
