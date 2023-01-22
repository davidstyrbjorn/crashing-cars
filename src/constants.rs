use crate::prelude::*;

pub const WINDOW_SIZE: f32 = 1800.0;
pub const WINDOW_HEIGHT: f32 = WINDOW_SIZE / 2.0;
pub const WINDOW_WIDTH: f32 = WINDOW_SIZE;
pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);

pub const PIXELS_PER_METER: f32 = 250.0;

pub const MOVE_SPEED: f32 = 1500.0;
pub const ANGULAR_SPEED: f32 = 25.0;
pub const ANGULAR_DEGRADE: f32 = 0.3;

pub const TERMINAL_FORCE: f32 = 20.0;
pub const BALL_RADIUS: f32 = 20.0;

pub const PROJECTILE_SPEED: f32 = 2000.0;
pub const PROJECTILE_LIFE_LENGTH: u128 = 200;
pub const PROJECTILE_KNOCKBACK: f32 = 25.0;

pub const GOAL_KEEPER_WIDTH: f32 = 10.0;
pub const GOAL_KEEPER_HEIGHT: f32 = 60.0;

pub const MODIFICATION_COUNT: usize = 3;

pub const HAZARD_RADIUS: f32 = 60.0;
pub const HAZARD_ROTATION_SPEED: f32 = 20.0;
