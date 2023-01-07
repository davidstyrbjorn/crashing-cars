mod ball;
mod player_move;
mod score_text;
mod setup_game;
mod timer;

use ball::ball;
use player_move::player_move;
use score_text::score_text;
use setup_game::setup_game;
use timer::timer;

use crate::prelude::*;

pub fn setup_systems_for_game(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::InGame)
            .with_system(player_move)
            .with_system(timer)
            .with_system(ball)
            .with_system(score_text),
    )
    .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game));
}
