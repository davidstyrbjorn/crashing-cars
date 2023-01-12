use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::{
    na::{Point, Point2},
    rapier::prelude::ColliderBuilder,
};
use leafwing_input_manager::user_input::InputKind;

use crate::prelude::*;

pub fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    println!("SPAWNING BALLS");
    commands
        .spawn((
            Ball,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::AQUAMARINE)),
                transform: Transform::from_translation(Vec3::new(0., 0.0, 0.0)),
                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::ball(BALL_RADIUS),
            Restitution::coefficient(0.5),
            Damping {
                linear_damping: 0.4,
                angular_damping: 0.0,
            },
            Velocity::zero(),
            ColliderMassProperties::Mass(0.025),
        ))
        .id()
}

pub fn spawn_player(
    commands: &mut Commands,
    left_right: (KeyCode, KeyCode),
    up_down: (KeyCode, KeyCode),
    select: KeyCode,
    spawn_position: Vec3,
    team: Team,
) {
    commands.spawn((
        Player {
            spawn_position,
            score: 0,
            team,
        },
        BaseStats::new(),
        SpriteBundle {
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
        },
        InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(
                    VirtualDPad {
                        left: InputKind::Keyboard(left_right.0),
                        right: InputKind::Keyboard(left_right.1),
                        down: InputKind::Keyboard(KeyCode::Numpad3),
                        up: InputKind::Keyboard(KeyCode::Numpad4),
                    },
                    Action::Rotate,
                )
                .insert(
                    VirtualDPad {
                        left: InputKind::Keyboard(KeyCode::Numpad3),
                        right: InputKind::Keyboard(KeyCode::Numpad0),
                        up: InputKind::Keyboard(up_down.0),
                        down: InputKind::Keyboard(up_down.1),
                    },
                    Action::Move,
                )
                .insert(select, Action::Select)
                .build(),
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
        Restitution::coefficient(0.4),
        Damping {
            linear_damping: 3.5,
            angular_damping: 0.0,
        },
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        },
        Velocity::zero(),
    ));
}

pub fn spawn_level_box(commands: &mut Commands) {
    const WIDTH: f32 = 10.0;
    const OPENING: f32 = (BALL_RADIUS * 2.0) * 8.0;

    // Spawn walls around the level
    // left-top
    commands.spawn((
        Collider::cuboid(WIDTH, (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0)),
        TransformBundle::from(Transform::from_xyz(
            -(WINDOW_WIDTH / 2.0) + WIDTH,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )),
    ));

    // left-bottom
    commands.spawn((
        Collider::cuboid(WIDTH, (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0)),
        TransformBundle::from(Transform::from_xyz(
            -(WINDOW_WIDTH / 2.0) + WIDTH,
            -WINDOW_HEIGHT / 2.0,
            0.0,
        )),
    ));

    // right-top
    commands.spawn((
        Collider::cuboid(WIDTH, (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0)),
        TransformBundle::from(Transform::from_xyz(
            (WINDOW_WIDTH / 2.0) - WIDTH,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )),
    ));

    // right-bottom
    commands.spawn((
        Collider::cuboid(WIDTH, (WINDOW_HEIGHT / 2.0) - (OPENING / 2.0)),
        TransformBundle::from(Transform::from_xyz(
            (WINDOW_WIDTH / 2.0) - WIDTH,
            -WINDOW_HEIGHT / 2.0,
            0.0,
        )),
    ));

    // top
    commands.spawn((
        Collider::cuboid(WINDOW_WIDTH, WIDTH),
        TransformBundle::from(Transform::from_xyz(0.0, (WINDOW_HEIGHT / 2.0) - WIDTH, 0.0)),
    ));

    // bottom
    commands.spawn((
        Collider::cuboid(WINDOW_WIDTH, WIDTH),
        TransformBundle::from(Transform::from_xyz(
            0.0,
            (-WINDOW_HEIGHT / 2.0) + WIDTH,
            0.0,
        )),
    ));

    // Corner colliders
    let positions = vec![(1.0, 1.0), (-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0)];
    positions.iter().for_each(|p| {
        commands.spawn((
            Collider::round_cuboid(2.0, 2.0, 0.8),
            TransformBundle::from(Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) * p.0,
                WINDOW_HEIGHT / 2.0 * p.1,
                0.0,
            )),
        ));
    });
}
