use crate::prelude::*;

pub fn calculate_score_for_player(player: &Player, score: (u8, u8)) -> u8 {
    let scored = match player.team {
        Team::Red => score.0 > score.1,
        Team::Blue => score.1 < score.1,
    };

    if scored {
        return 1;
    } else {
        return 0;
    }
}

pub fn on_round_end(
    mut commands: Commands,
    mut score_counter: ResMut<ScoreCounter>,
    mut draft_resource: ResMut<DraftResource>,
    mut query: Query<(&mut Player, Entity)>,
) {
    // See score for round, assign score to player
    query.iter_mut().for_each(|(mut player, _)| {
        (*player).score += calculate_score_for_player(&player, score_counter.score);
    });

    // TODO Make this the actual orrect order
    query
        .iter()
        .map(|(player, entity)| (player.score, entity))
        .for_each(|(_, entity)| {
            draft_resource.pick_order.push_back(entity);
        });

    // TODO: dangerous-ish unwrap here
    let first = draft_resource.pick_order.pop_back().unwrap();
    commands.entity(first).insert(CurrentlyPicking);

    // Reset
    draft_resource.current_idx = 0;
    score_counter.score.0 = 0;
    score_counter.score.1 = 0;
}
