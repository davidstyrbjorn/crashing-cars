use crate::prelude::*;
use rand::{distributions::Uniform, thread_rng, Rng};
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

// Struct for storing modifications, load them from ron file and store in struct

// Each modification to the game will be represented by a enum state
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum ModificationType {
    GoalKeeper {
        to: Entity,
    },
    IncreaseSpeed {
        to: Entity,
    },
    DecreaseDegrade {
        to: Entity,
    },
    Turret {
        to: Entity,
    },
    Inverted {
        to: Entity,
        number_of_rounds: Entity,
    },
    ModifyField {
        counter: u32,
    },
    AddHazard {
        counter: u32,
    },
    ModifyCar {
        counter: u32,
    },
}

// On modification pickup we can basically add a new component to the given entity
// For example the "Map" entity might get a Hazard component when that modification is picked up
// Or the player entity might get a Turret component when that modification is picked up
// Simple as that, each component will also then need some state

#[derive(Clone, Deserialize, Debug)]
pub struct Modification {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct Modifications {
    pub modifications: Vec<Modification>,
}

impl Modifications {
    pub fn load() -> Self {
        let file = File::open("assets/modifications.ron").expect("Failed to load game data");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn get_modification(&mut self) -> Modification {
        let mut rng = thread_rng();
        let roller = Uniform::new_inclusive(0, self.modifications.len() - 1);
        let mut roll_die = (&mut rng).sample_iter(roller);
        let idx = roll_die.next().unwrap();
        self.modifications.remove(idx).clone()
    }

    pub fn modification_picked(modification_type: ModificationType, mut commands: Commands) {
        match modification_type {
            ModificationType::GoalKeeper { to } => {
                commands.entity(to).insert(GoalKeeper);
            }
            ModificationType::IncreaseSpeed { to } => {
                commands.entity(to).insert(LinearSpeedModifier(1.2));
                commands.entity(to).insert(AngularSpeedModifier(1.2));
            }
            ModificationType::DecreaseDegrade { to } => {
                commands.entity(to).insert(AngularDegradeModifier(0.5));
            }
            ModificationType::Turret { to } => {
                println!("G??DIGT");
            }
            ModificationType::Inverted {
                to,
                number_of_rounds,
            } => {
                println!("G??DIGT");
            }
            ModificationType::ModifyField { counter } => {
                println!("G??DIGT");
            }
            ModificationType::AddHazard { counter } => {
                println!("G??DIGT");
            }
            ModificationType::ModifyCar { counter } => {
                println!("G??DIGT");
            }
        }
    }
}
