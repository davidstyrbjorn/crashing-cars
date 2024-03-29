use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum Label {
    Pre,
    Apply,
    Main,
}

pub struct IntroPlugin;
pub struct MenuPlugin;
pub struct GamePlugin;
pub struct ModificationPlugin;
pub struct SharedPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IntroTimerResource {
            timer: Timer::from_seconds(4.0, TimerMode::Repeating),
        })
        .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(setup_intro))
        .add_system_set(SystemSet::on_update(GameState::Intro).with_system(run_intro));
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Pre)
                .before(Label::Main)
                .with_system(new_round_player)
                .with_system(new_round_ball)
                .with_system(new_round_timer)
                .with_system(new_round_score_counter)
                .with_system(new_round_turret)
                .with_system(reset_base_system)
                .with_system(projectile)
                .with_system(boost),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Apply)
                .before(Label::Main)
                .after(Label::Pre)
                .with_system(modifier_angular_speed)
                .with_system(modifier_angular_degrade)
                .with_system(modifier_linear_speed)
                .with_system(turret)
                .with_system(projectile_collision_player)
                .with_system(projectile_collision_anything)
                .with_system(health_reset_position)
                .with_system(camera_shake),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Main)
                .after(Label::Apply)
                .with_system(ball_score)
                .with_system(player_move)
                .with_system(score_text)
                .with_system(timer)
                .with_system(prepare_timer)
                .with_system(projectile_event)
                .with_system(goal_keeper)
                .with_system(ball_scored_reset_players)
                .with_system(ball_scored_update_score)
                .with_system(ball_scored_reset_ball)
                .with_system(ball_scored_reset_prepare_timer)
                .with_system(hazard)
                .with_system(boost_pickup_spot)
                .with_system(boost_pickup_spot_visual),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(setup_game)
                .with_system(despawn_entities::<OnMainMenu>)
                .with_system(despawn_entities::<OnModification>)
                .with_system(despawn_entities::<Projectile>),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InGame)
                .with_system(on_round_end)
                .with_system(inverted_controls_degrade),
        );
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(press_to_start_game),
        )
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
    }
}

impl Plugin for ModificationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InModification).with_system(setup_modification),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InModification)
                .with_system(sliding_window)
                .with_system(modification_input)
                .with_system(highlight_modification_element)
                .with_system(draft_pick)
                .with_system(modification_element_remove),
        );
    }
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shared);
    }
}
