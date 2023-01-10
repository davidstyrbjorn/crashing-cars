use crate::prelude::*;

pub fn press_to_start_game(
    mut app_state: ResMut<State<GameState>>,
    query: Query<&ActionState<Action>>,
) {
    if query.single().pressed(Action::StartGame) {
        // TODO: Dangerous unwrap
        app_state.set(GameState::InGame).unwrap();
    }
}
