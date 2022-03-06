use specs::prelude::*;

pub struct DeathSystem;

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, crate::Map>,
        WriteExpect<'a, crate::RunState>,
        ReadStorage<'a, crate::Position>,
        ReadStorage<'a, crate::Health>,
        ReadStorage<'a, crate::MultiTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player, mut map, mut run_state, positions, healths, multitiles) = data;
        let mut dead = Vec::new();

        for (ent, pos, health, multis) in
            (&entities, &positions, &healths, (&multitiles).maybe()).join()
        {
            if health.current <= 0 {
                if ent != *player {
                    dead.push(ent);
                    map.untrack_creature(pos.as_point(), multis);
                } else {
                    *run_state = crate::RunState::Dead;
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
