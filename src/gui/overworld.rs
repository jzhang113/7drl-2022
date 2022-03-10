use rltk::{Algorithm2D, Rltk, RGB};
use specs::World;

pub fn screen(ecs: &World, ctx: &mut Rltk) {
    ctx.print(1, 1, "Hello overmap");
}
