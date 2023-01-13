use crate::prelude::*;

// Message for when draft pick was made
pub struct DraftPickEvent {
    pub modification: Modification,
    pub who: Entity,
    pub idx: usize,
}

// Message for when modification state is done with its shit
pub struct ModificationDone;
