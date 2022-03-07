use specs::prelude::*;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, crate::Map>,
        ReadStorage<'a, crate::MultiTile>,
        WriteStorage<'a, crate::Position>,
        WriteStorage<'a, crate::MoveIntent>,
        WriteStorage<'a, crate::Viewshed>,
        WriteStorage<'a, crate::Facing>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, multis, mut positions, mut movements, mut viewsheds, mut facings) =
            data;

        for (ent, pos, movement, multi, viewshed, facing) in (
            &entities,
            &mut positions,
            &movements,
            (&multis).maybe(),
            (&mut viewsheds).maybe(),
            (&mut facings).maybe(),
        )
            .join()
        {
            let new_pos = movement.loc;

            if let Some(facing) = facing {
                let dx = new_pos.x - pos.x;
                let dy = new_pos.y - pos.y;

                facing.direction = if dx.abs() > dy.abs() {
                    match dx.signum() {
                        1 => crate::Direction::E,
                        -1 => crate::Direction::W,
                        _ => unreachable!(), // if dx.signum is 0, dx = 0, but we can't be in this branch in that case
                    }
                } else {
                    match dy.signum() {
                        1 => crate::Direction::S,
                        -1 => crate::Direction::N,
                        _ => crate::Direction::N,
                    }
                }
            }

            // update the position if we successfully moved to new_pos
            if map.move_creature(ent, pos.as_point(), new_pos, multi) {
                pos.x = new_pos.x;
                pos.y = new_pos.y;

                if let Some(viewshed) = viewshed {
                    viewshed.dirty = true;
                }
            }
        }

        movements.clear();
    }
}
