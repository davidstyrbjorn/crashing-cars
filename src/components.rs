use crate::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Team {
    Red = 0,  // left
    Blue = 1, // right
}

#[derive(Component, Clone)]
pub struct Player {
    pub spawn_position: Vec3,
    pub spawn_rotation: f32,
    pub score: u8,
    pub team: Team,
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
pub struct OnModification;

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
pub struct GoalKeeper {
    pub team: Team,
}

#[derive(Component)]
pub struct ModificationElement {
    pub order: usize,
}

#[derive(Component)]
pub struct CurrentlyPicking;

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub spawned_at_time: u128,
}

#[derive(Component)]
pub struct ModifyField(pub u32);

#[derive(Component)]
pub struct ModifyCar(pub u32);

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct IntroText {
    pub order: usize,
    pub time: u32,
}

#[derive(Component)]
pub struct PrepareTimerText;

#[derive(Component)]
pub struct InvertedControls(pub u8);

#[derive(Component)]
pub struct Hazard;
