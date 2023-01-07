use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

pub fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::AQUAMARINE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::ball(BALL_RADIUS))
        .insert(Restitution::coefficient(0.8))
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 0.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Velocity::zero())
        .id()
}

pub fn spawn_player(
    commands: &mut Commands,
    input: impl Into<UserInput>,
    spawn_position: Vec3,
) -> Entity {
    commands
        .spawn()
        .insert(Player { spawn_position })
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
        .insert(Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0))
        .insert(Restitution::coefficient(0.4))
        .insert(Damping {
            linear_damping: 3.5,
            angular_damping: 0.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Velocity::zero())
        .id()
}

pub fn spawn_level_box(commands: &mut Commands) {
    const WIDTH: f32 = 10.0;
    const OPENING: f32 = (BALL_RADIUS * 2.0) * 8.0;

    // Spawn walls around the level
    // left-top
    commands
        .spawn()
        .insert(Collider::cuboid(
            WIDTH,
            (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0),
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -(WINDOW_WIDTH / 2.0) + WIDTH,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )));
    // left-bottom
    commands
        .spawn()
        .insert(Collider::cuboid(
            WIDTH,
            (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0),
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -(WINDOW_WIDTH / 2.0) + WIDTH,
            -WINDOW_HEIGHT / 2.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::round_cuboid(10.0, 10.0, 0.2))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )));

    // right-top
    commands
        .spawn()
        .insert(Collider::cuboid(
            WIDTH,
            (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0),
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            (WINDOW_WIDTH / 2.0) - WIDTH,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )));
    // right-bottom
    commands
        .spawn()
        .insert(Collider::cuboid(
            WIDTH,
            (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0),
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            (WINDOW_WIDTH / 2.0) - WIDTH,
            -WINDOW_HEIGHT / 2.0,
            0.0,
        )));

    // top
    commands
        .spawn()
        .insert(Collider::cuboid(WINDOW_WIDTH, WIDTH))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            (WINDOW_HEIGHT / 2.0) - WIDTH,
            0.0,
        )));

    // bottom
    commands
        .spawn()
        .insert(Collider::cuboid(WINDOW_WIDTH, WIDTH))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            (-WINDOW_HEIGHT / 2.0) + WIDTH,
            0.0,
        )));
}
