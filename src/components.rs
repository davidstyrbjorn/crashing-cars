use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub spawn_position: Vec3,
    pub score: u8,
}

#[derive(Component)]
pub struct PhysicsDebug;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallReset; // Flag component

#[derive(Component)]
pub struct OnMainMenu;

#[derive(Component)]
pub struct SlidingWindow(pub f32);

// In here we should store all sorts of stats we want to modify in the game
#[derive(Component)]
pub struct BaseStats {
    pub linear_speed: f32,
    pub angular_speed: f32,
    pub angular_degrade: f32,
}

impl BaseStats {
    pub fn new() -> Self {
        BaseStats {
            linear_speed: MOVE_SPEED,
            angular_speed: ANGULAR_SPEED,
            angular_degrade: ANGULAR_DEGRADE,
        }
    }
}

#[derive(Component)]
pub struct LinearSpeedModifier(pub f32);

#[derive(Component)]
pub struct AngularSpeedModifier(pub f32);

#[derive(Component)]
pub struct AngularDegradeModifier(pub f32);

#[derive(Component)]
pub struct GoalKeeper;
