use std::time::Duration;

use crate::prelude::*;

pub fn boost(mut commands: Commands, timer: Res<Time>, mut query: Query<(&mut Boost, Entity)>) {
    for (mut boost, entity) in query.iter_mut() {
        boost.0.tick(timer.delta());
        if boost.0.finished() {
            println!("REMOVE BOOST");
            commands.entity(entity).remove::<Boost>();
        }
    }
}

pub fn boost_pickup_spot(
    rapier_context: Res<RapierContext>,
    players: Res<Players>,
    time: Res<Time>,
    mut query: Query<(&mut BoostPickupSpot, Entity)>,
    mut commands: Commands,
) {
    for (mut boost_pickup_spot, entity) in query.iter_mut() {
        boost_pickup_spot.timer.tick(time.delta());
        if boost_pickup_spot.timer.finished() && !boost_pickup_spot.has_boost {
            boost_pickup_spot.has_boost = true;
        }

        // if this spot has boost check for player collision
        if boost_pickup_spot.has_boost {
            // Also check if a player collides with us
            players.0.iter().for_each(|player| {
                if let Some(_) = rapier_context.intersection_pair(entity, *player) {
                    boost_pickup_spot.timer.reset();
                    boost_pickup_spot.has_boost = false;
                    commands.entity(*player).insert(Boost(Timer::new(
                        Duration::from_millis(BOOST_DURATION),
                        TimerMode::Once,
                    )));
                }
            });
        }
    }
}

pub fn boost_pickup_spot_visual(mut query: Query<(&BoostPickupSpot, &mut Sprite)>) {
    query
        .iter_mut()
        .for_each(|(boost_pickup_spot, mut sprite)| {
            let mut color = Color::WHITE;
            if !boost_pickup_spot.has_boost {
                color.set_a(0.0);
            }
            sprite.color = color;
        });
}
