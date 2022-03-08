use specs::prelude::*;
use std::collections::HashMap;

pub struct PartMoveSystem;

impl<'a> System<'a> for PartMoveSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, crate::Position>,
        WriteStorage<'a, crate::PartMoveIntent>,
        WriteStorage<'a, crate::MultiTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, mut part_moves, mut multitiles) = data;

        for (_, _, moves, multis) in (&entities, &positions, &part_moves, &mut multitiles).join() {
            for (i, dir) in moves.part_delta.iter().enumerate() {
                let mut new_symbol_map = HashMap::new();
                for (part_pos, symbol) in &multis.part_list[i].symbol_map {
                    new_symbol_map.insert(*part_pos + *dir, *symbol);
                }

                multis.part_list[i].symbol_map = new_symbol_map;
            }
        }

        part_moves.clear();
    }
}
