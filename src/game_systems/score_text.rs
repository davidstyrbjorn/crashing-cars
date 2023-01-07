use crate::prelude::*;

pub fn score_text(mut query: Query<&mut Text, With<ScoreText>>, score_counter: Res<ScoreCounter>) {
    let mut text = query.single_mut();

    text.sections[0].value =
        format!("{} - {}", score_counter.score.0, score_counter.score.1).into();
}
