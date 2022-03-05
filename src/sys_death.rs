use super::{Health, Map, Position, RunState};
use specs::prelude::*;

pub struct DeathSystem;

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, RunState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Health>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player, mut map, mut run_state, positions, healths) = data;
        let mut dead = Vec::new();

        for (ent, pos, health) in (&entities, &positions, &healths).join() {
            if health.current <= 0 {
                if ent != *player {
                    dead.push(ent);
                    map.untrack_creature(rltk::Point::new(pos.x, pos.y));
                } else {
                    *run_state = RunState::Dead;
                }
            }
        }

        for victim in dead {
            entities
                .delete(victim)
                .expect("Failed to remove dead entity");
        }
    }
}
