use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub spawn_position: Vec3,
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
