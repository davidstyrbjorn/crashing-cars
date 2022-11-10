use crate::prelude::*;

pub fn spawn_player(
    commands: &mut Commands,
    input: impl Into<UserInput>,
    spawn_position: Vec3,
) -> Entity {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE.x, PLAYER_SIZE.y)),
                ..Default::default()
            },
            transform: Transform {
                translation: spawn_position,
                rotation: Quat::default(),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..Default::default()
        })
        // Insert an input bundle, bind Move to WASD, ARROWS, DPAD, STICK etc
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default().insert(input, Action::Move).build(),
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::ball(PLAYER_SIZE.x / 2.0))
        .insert(Restitution::coefficient(0.8))
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 0.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .id()
}

pub fn spawn_level(commands: &mut Commands) {
    const WIDTH: f32 = 10.0;

    // Spawn walls around the level
    commands
        .spawn()
        .insert(Collider::cuboid(WIDTH, WINDOW_HEIGHT))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            (-WINDOW_WIDTH / 2.0) - WIDTH,
            0.0,
            0.0,
        )));

    commands
        .spawn()
        .insert(Collider::cuboid(WIDTH, WINDOW_HEIGHT))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            (WINDOW_WIDTH / 2.0) + WIDTH,
            0.0,
            0.0,
        )));

    commands
        .spawn()
        .insert(Collider::cuboid(WINDOW_WIDTH, WIDTH))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            (WINDOW_HEIGHT / 2.0) + WIDTH,
            0.0,
        )));

    commands
        .spawn()
        .insert(Collider::cuboid(WINDOW_WIDTH, WIDTH))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            (-WINDOW_HEIGHT / 2.0) - WIDTH,
            0.0,
        )));
}
