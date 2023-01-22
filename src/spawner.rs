use std::f32::consts::PI;

use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::rapier::prelude::{ColliderBuilder, ColliderFlags};
use leafwing_input_manager::user_input::InputKind;

use crate::prelude::*;

pub fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            Ball,
            SpriteBundle {
                texture: asset_server.load("blue.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0)),
                    ..default()
                },
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
            // ColliderMassProperties::Mass(0.025),
        ))
        .id()
}

pub fn spawn_player(
    commands: &mut Commands,
    left_right: (KeyCode, KeyCode),
    up_down: (KeyCode, KeyCode),
    select: KeyCode,
    turret: KeyCode,
    spawn_position: Vec3,
    team: Team,
) -> Entity {
    let mut rotation_z = PI / 2.0;
    let mut color = Color::BLUE;
    if team == Team::Red {
        rotation_z = -PI / 2.0;
        color = Color::RED;
    }
    color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    commands
        .spawn((
            // Turret,
            Health(10),
            Player {
                spawn_position,
                score: 0,
                team,
                spawn_rotation: rotation_z,
            },
            BaseStats::new(),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLAYER_SIZE.x, PLAYER_SIZE.y)),
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: spawn_position,
                    rotation: Quat::from_rotation_z(rotation_z),
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
                    .insert(turret, Action::Turret)
                    .build(),
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            // Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
            Collider::triangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(PLAYER_SIZE.x, 0.0),
                Vec2::new(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y),
            ),
            Restitution::coefficient(0.4),
            Damping {
                linear_damping: 3.5,
                angular_damping: 0.0,
            },
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0,
            },
            Velocity::zero(),
        ))
        .id()
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

pub fn spawn_goal_keeper(commands: &mut Commands, team: Team) {
    let mut spawn_position = Vec3::new(-WINDOW_WIDTH / 2.2, 0.0, 0.0);

    if team == Team::Blue {
        spawn_position.x = spawn_position.x * -1.0;
    }

    commands.spawn((
        GoalKeeper { team },
        TransformBundle::from_transform(Transform::from_translation(spawn_position)),
        Collider::cuboid(GOAL_KEEPER_WIDTH, GOAL_KEEPER_HEIGHT),
    ));
}

pub fn spawn_modify_field(commands: &mut Commands, counter: u32) {
    const WIDTH: f32 = 100.0;
    const HEIGHT: f32 = 10.0;

    commands.spawn((
        Collider::cuboid(HEIGHT, WIDTH),
        TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Collider::cuboid(HEIGHT, WIDTH),
        TransformBundle::from(Transform::from_xyz(100.0, 0.0, 0.0)),
    ));
}

pub fn spawn_projectile(commands: &mut Commands, origin: Vec3, rotation: Quat, time: Time) {
    let direction = rotation * Vec3::Y;
    let offset = PLAYER_SIZE.x * 1.5;
    let matrix = Mat4::from_translation(origin + direction * offset);

    commands.spawn((
        Projectile {
            direction,
            spawned_at_time: time.elapsed().as_millis(),
        },
        TransformBundle::from_transform(Transform::from_matrix(matrix)),
        Collider::ball(6.0),
        RigidBody::Fixed,
    ));
}
