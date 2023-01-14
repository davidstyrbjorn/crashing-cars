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

pub const GOAL_KEEPER_WIDTH: f32 = PLAYER_SIZE.x;
pub const GOAL_KEEPER_HEIGHT: f32 = PLAYER_SIZE.x * 2.0;

pub const MODIFICATION_COUNT: usize = 3;
