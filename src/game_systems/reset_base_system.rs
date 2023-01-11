use crate::prelude::*;

pub fn reset_base_system(mut query: Query<&mut BaseStats>) {
    query.iter_mut().for_each(|mut base_stat| {
        *base_stat = BaseStats::new();
    });
}
