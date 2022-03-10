use super::common::*;
use super::MapBuilder;
use crate::*;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub spawn_mode: DrunkSpawnMode,
    pub drunken_lifetime: i32,
    pub floor_percent: f32,
    pub digger_size: i32,
}

pub struct DrunkardsWalkBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<usize>>,
    settings: DrunkardSettings,
}

impl MapBuilder for DrunkardsWalkBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator) -> Map {
        self.build(rng);
        self.get_map()
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.depth);
        }
    }

    fn take_snapshot(&mut self) {
        if super::SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.known_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

impl DrunkardsWalkBuilder {
    #[allow(dead_code)]
    pub fn new(
        width: i32,
        height: i32,
        depth: i32,
        rng: &mut rltk::RandomNumberGenerator,
        settings: DrunkardSettings,
    ) -> Self {
        Self {
            map: Map::new(width, height, depth, rng),
            starting_position: Position { x: 0, y: 0 },
            depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            settings,
        }
    }

    pub fn open_area(
        width: i32,
        height: i32,
        depth: i32,
        rng: &mut rltk::RandomNumberGenerator,
    ) -> Self {
        Self {
            map: Map::new(width, height, depth, rng),
            starting_position: Position { x: 0, y: 0 },
            depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 1000,
                floor_percent: 0.5,
                digger_size: 4,
            },
        }
    }

    pub fn open_halls(
        width: i32,
        height: i32,
        depth: i32,
        rng: &mut rltk::RandomNumberGenerator,
    ) -> Self {
        Self {
            map: Map::new(width, height, depth, rng),
            starting_position: Position { x: 0, y: 0 },
            depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                digger_size: 3,
            },
        }
    }

    pub fn winding_passages(
        width: i32,
        height: i32,
        depth: i32,
        rng: &mut rltk::RandomNumberGenerator,
    ) -> Self {
        Self {
            map: Map::new(width, height, depth, rng),
            starting_position: Position { x: 0, y: 0 },
            depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                digger_size: 2,
            },
        }
    }

    fn build(&mut self, rng: &mut rltk::RandomNumberGenerator) {
        // Set a central starting point
        self.starting_position = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        };
        let start_idx = self
            .map
            .get_index(self.starting_position.x, self.starting_position.y);
        self.map.tiles[start_idx] = TileType::Floor;

        let total_tiles = self.map.width * self.map.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self
            .map
            .tiles
            .iter()
            .filter(|a| **a == TileType::Floor)
            .count();
        let mut digger_count = 0;
        let mut active_digger_count = 0;
        while floor_tile_count < desired_floor_tiles && digger_count < 1000 {
            let mut did_something = false;
            let mut drunk_x;
            let mut drunk_y;
            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = self.starting_position.x;
                    drunk_y = self.starting_position.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = self.starting_position.x;
                        drunk_y = self.starting_position.y;
                    } else {
                        drunk_x =
                            rng.roll_dice(1, self.map.width - self.settings.digger_size - 2) + 1;
                        drunk_y =
                            rng.roll_dice(1, self.map.height - self.settings.digger_size - 2) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                for dx in 0..=self.settings.digger_size {
                    for dy in 0..=self.settings.digger_size {
                        let drunk_idx = self.map.get_index(drunk_x + dx, drunk_y + dy);
                        if self.map.tiles[drunk_idx] == TileType::Wall {
                            did_something = true;
                        }
                        self.map.tiles[drunk_idx] = TileType::DownStairs;
                    }
                }

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < self.map.width - self.settings.digger_size - 1 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < self.map.height - self.settings.digger_size - 1 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }
            if did_something {
                self.take_snapshot();
                active_digger_count += 1;
            }

            digger_count += 1;
            for t in self.map.tiles.iter_mut() {
                if *t == TileType::DownStairs {
                    *t = TileType::Floor;
                }
            }
            floor_tile_count = self
                .map
                .tiles
                .iter()
                .filter(|a| **a == TileType::Floor)
                .count();
        }
        rltk::console::log(format!(
            "{} dwarves gave up their sobriety, of whom {} actually found a wall.",
            digger_count, active_digger_count
        ));

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        // Place the stairs
        self.map.level_exit = exit_tile;
        self.take_snapshot();

        // Now we build a noise map for use in spawning entities later
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, rng);
    }
}
