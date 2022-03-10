pub mod bsp;
mod common;
pub mod drunk_walk;

const SHOW_MAPGEN_VISUALIZER: bool = false;

pub trait MapBuilder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator) -> super::Map;
    fn spawn_entities(&mut self, ecs: &mut super::World);
    fn get_map(&self) -> super::Map;
    fn get_starting_position(&self) -> super::Position;
    fn get_snapshot_history(&self) -> Vec<super::Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(width: i32, height: i32, depth: i32) -> Box<dyn MapBuilder> {
    let mut rng = rltk::RandomNumberGenerator::new();
    let builder_type = rng.roll_dice(1, 3);
    println!("Building map type {}", builder_type);

    let builder = match builder_type {
        //1 => Box::new(BspDungeonBuilder::new(new_depth)),
        // 2 => Box::new(BspInteriorBuilder::new(new_depth)),
        // 3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        1 => drunk_walk::DrunkardsWalkBuilder::open_area(width, height, depth, &mut rng),
        2 => drunk_walk::DrunkardsWalkBuilder::open_halls(width, height, depth, &mut rng),
        _ => drunk_walk::DrunkardsWalkBuilder::winding_passages(width, height, depth, &mut rng),
        //_ => Box::new(SimpleMapBuilder::new(new_depth)),
    };

    Box::new(builder)
}
