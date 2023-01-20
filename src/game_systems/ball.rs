use crate::prelude::*;

fn ball_scored(position: Vec3) -> Option<Team> {
    let bounds = (
        position.x > WINDOW_WIDTH / 2.0,
        position.x < -WINDOW_WIDTH / 2.0,
    );
    if bounds.0 {
        return Some(Team::Red);
    } else if bounds.1 {
        return Some(Team::Blue);
    }

    return None;
}

// Checks if ball has scored if so push event
pub fn ball_score(
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut event_writer: EventWriter<GoalEvent>,
) {
    // Check if ball transform is valid goal position
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        if let Some(team) = ball_scored(transform.translation) {
            event_writer.send(GoalEvent { team });
        }
    }
}

pub fn ball_scored_reset_players(
    mut event_reader: EventReader<GoalEvent>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for _ in event_reader.iter() {
        for (mut transform, player) in query.iter_mut() {
            transform.translation = player.spawn_position;
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, player.spawn_rotation);
        }
    }
}

pub fn ball_scored_update_score(
    mut event_reader: EventReader<GoalEvent>,
    mut score_counter: ResMut<ScoreCounter>,
) {
    for goal_event in event_reader.iter() {
        if goal_event.team == Team::Red {
            score_counter.score.0 += 1;
        }
        if goal_event.team == Team::Blue {
            score_counter.score.1 += 1;
        }
    }
}

pub fn ball_scored_reset_ball(
    mut event_reader: EventReader<GoalEvent>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for _ in event_reader.iter() {
        for (mut transform, mut velocity) in query.iter_mut() {
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            velocity.linvel = Vec2::new(0.0, 0.0);
        }
    }
}
