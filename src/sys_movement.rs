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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, multis, mut positions, mut movements, mut viewsheds) = data;

        for (ent, pos, movement, multi, viewshed) in (
            &entities,
            &mut positions,
            &movements,
            (&multis).maybe(),
            (&mut viewsheds).maybe(),
        )
            .join()
        {
            let new_pos = movement.loc;

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
