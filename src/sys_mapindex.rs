use specs::prelude::*;

pub struct MapIndexSystem;

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (
        WriteExpect<'a, crate::Map>,
        ReadStorage<'a, crate::Position>,
        ReadStorage<'a, crate::BlocksTile>,
        ReadStorage<'a, crate::MultiTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers, multis) = data;

        // Fix tiles that should be blocked
        map.set_blocked_tiles();
        for (pos, _, multi) in (&positions, &blockers, (&multis).maybe()).join() {
            let index = map.get_index(pos.x, pos.y);
            map.blocked_tiles[index] = true;

            if let Some(multi) = multi {
                for part in &multi.part_list {
                    for part_pos in part.symbol_map.keys() {
                        let part_pos_index = map.get_index(pos.x + part_pos.x, pos.y + part_pos.y);
                        map.blocked_tiles[part_pos_index] = true;
                    }
                }
            }
        }
    }
}
