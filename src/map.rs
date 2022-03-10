use crate::spawner;
use rltk::{Algorithm2D, BaseMap, Point, Rect};
use specs::Entity;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
struct SearchArgs {
    search_entity: Option<Entity>,
    multi_component: Option<Vec<crate::MonsterPart>>,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub camera: crate::Camera,
    pub color_map: Vec<rltk::RGB>,
    pub item_map: HashMap<usize, Entity>,
    pub creature_map: HashMap<usize, Entity>,
    pub known_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub level_exit: usize,
    search_args: SearchArgs,
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Manhattan.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl Map {
    pub fn get_index(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn set_blocked_tiles(&mut self) {
        for (index, tile) in self.tiles.iter_mut().enumerate() {
            let is_blocked = *tile == TileType::Wall;
            self.blocked_tiles[index] = is_blocked;
        }
    }

    fn is_tile_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }

        let index = self.get_index(x, y);

        // TODO: multi-tile bodies can still walk into players since player doesn't have a collision
        // non-blocked tiles are always valid
        if !self.blocked_tiles[index] {
            return true;
        }

        // walls are always invalid
        if self.tiles[index] == TileType::Wall {
            return false;
        }

        // blocked tiles can be valid if they belong to the search_entity (creatures are not blocked by themselves)
        let result = match self.search_args.search_entity {
            Some(search_entity) => match self.creature_map.get(&index) {
                Some(map_entity) => *map_entity == search_entity,
                None => false,
            },
            None => false,
        };

        result
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if !self.is_tile_valid(x, y) {
            return false;
        }

        // if search_entity is a multi-tile entity, test the actual move first
        if let Some(multitiles) = &self.search_args.multi_component {
            for part in multitiles {
                for part_pos in part.symbol_map.keys() {
                    let new_x = x + part_pos.x;
                    let new_y = y + part_pos.y;

                    if !self.is_tile_valid(new_x, new_y) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn set_additional_args(
        &mut self,
        entity: Entity,
        multi_component: Option<&crate::MultiTile>,
    ) {
        self.search_args.search_entity = Some(entity);
        self.search_args.multi_component = multi_component.map(|comp| comp.part_list.clone());
    }

    pub fn is_exit_valid_for(
        &mut self,
        x: i32,
        y: i32,
        entity: Entity,
        multi_component: Option<&crate::MultiTile>,
    ) -> bool {
        self.set_additional_args(entity, multi_component);
        self.is_exit_valid(x, y)
    }

    pub fn get_available_exits_for(
        &mut self,
        idx: usize,
        entity: Entity,
        multi_component: Option<&crate::MultiTile>,
    ) -> rltk::SmallVec<[(usize, f32); 10]> {
        self.set_additional_args(entity, multi_component);
        self.get_available_exits(idx)
    }

    fn build_room(&mut self, room: Rect) {
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let index = self.get_index(x, y);
                self.tiles[index] = TileType::Floor;
                self.color_map[index] = crate::map_floor_color();
            }
        }

        self.rooms.push(room);
    }

    /// Create a hallway of TileType::Floor between the given start and end points
    /// The hallway will always be built horizontally from the start position and vertically from the end position
    fn build_hallway(&mut self, start: Point, end: Point) {
        let xrange;
        let yrange;

        if start.x > end.x {
            xrange = (end.x - start.x)..=0;
        } else {
            xrange = 0..=(end.x - start.x);
        }

        if start.y > end.y {
            yrange = 0..=(start.y - end.y);
        } else {
            yrange = (start.y - end.y)..=0;
        }

        for dx in xrange {
            let next_x = start.x + dx;
            let next_y = start.y;

            let index = self.get_index(next_x, next_y);
            self.tiles[index] = TileType::Floor;
            self.color_map[index] = crate::map_floor_color();
        }

        for dy in yrange {
            let next_x = end.x;
            let next_y = end.y + dy;

            let index = self.get_index(next_x, next_y);
            self.tiles[index] = TileType::Floor;
            self.color_map[index] = crate::map_floor_color();
        }
    }

    pub fn track_item(&mut self, data: Entity, point: Point) -> bool {
        let index = self.point2d_to_index(point);

        if self.item_map.get(&index).is_some() {
            false
        } else {
            self.item_map.insert(index, data);
            true
        }
    }

    pub fn untrack_item(&mut self, point: Point) -> Option<Entity> {
        let index = self.point2d_to_index(point);
        self.item_map.remove(&index)
    }

    fn update_multi_component(
        &mut self,
        entity: Entity,
        multi_component: &crate::MultiTile,
        point: Point,
        is_blocked: bool,
    ) {
        for part in &multi_component.part_list {
            for part_pos in part.symbol_map.keys() {
                let part_pos_index = self.point2d_to_index(*part_pos + point);
                self.blocked_tiles[part_pos_index] = is_blocked;

                if is_blocked {
                    self.creature_map.insert(part_pos_index, entity);
                } else {
                    self.creature_map.remove(&part_pos_index);
                }
            }
        }
    }

    pub fn track_creature(
        &mut self,
        data: Entity,
        point: Point,
        multi_component: Option<&crate::MultiTile>,
    ) -> bool {
        let index = self.point2d_to_index(point);

        if self.creature_map.get(&index).is_some() {
            false
        } else {
            self.blocked_tiles[index] = true;
            self.creature_map.insert(index, data);

            if let Some(multi_component) = multi_component {
                self.update_multi_component(data, &multi_component, point, true);
            }

            true
        }
    }

    pub fn untrack_creature(
        &mut self,
        point: Point,
        multi_component: Option<&crate::MultiTile>,
    ) -> Option<Entity> {
        let index = self.point2d_to_index(point);
        self.blocked_tiles[index] = false;
        let entity = self.creature_map.remove(&index);

        if let Some(entity) = entity {
            if let Some(multi_component) = multi_component {
                self.update_multi_component(entity, &multi_component, point, false);
            }
        }

        entity
    }

    // move a creature on the map, updating creature_map and blocked_tiles as needed
    // this does not update the position component
    // returns false if the move could not be completed
    pub fn move_creature(
        &mut self,
        creature: Entity,
        prev: Point,
        next: Point,
        multi_component: Option<&crate::MultiTile>,
    ) -> bool {
        let prev_index = self.point2d_to_index(prev);
        let next_index = self.point2d_to_index(next);

        // if the destination is blocked by something other than us, quit moving
        if !self.is_exit_valid_for(next.x, next.y, creature, multi_component) {
            return false;
        }

        self.creature_map.remove(&prev_index);
        self.blocked_tiles[prev_index] = false;
        if let Some(multi_component) = multi_component {
            self.update_multi_component(creature, multi_component, prev, false);
        }

        self.creature_map.insert(next_index, creature);
        self.blocked_tiles[next_index] = true;
        if let Some(multi_component) = multi_component {
            self.update_multi_component(creature, multi_component, next, true);
        }

        true
    }
}

pub fn build_rogue_map(
    width: i32,
    height: i32,
    depth: i32,
    rng: &mut rltk::RandomNumberGenerator,
) -> Map {
    let dim = (width * height).try_into().unwrap();
    let mut map = Map {
        tiles: vec![TileType::Wall; dim],
        rooms: vec![],
        width,
        height,
        depth,
        camera: crate::Camera {
            origin: rltk::Point::zero(),
        },
        color_map: (0..dim).map(|_| crate::map_wall_color(rng)).collect(),
        item_map: HashMap::new(),
        creature_map: HashMap::new(),
        known_tiles: vec![false; dim],
        visible_tiles: vec![false; dim],
        blocked_tiles: vec![false; dim],
        level_exit: 0,
        search_args: SearchArgs::default(),
    };

    const MAX_ROOMS: i32 = 12;
    const MIN_ROOM_WIDTH: i32 = 20;
    const MAX_ROOM_WIDTH: i32 = 50;
    const MIN_ROOM_HEIGHT: i32 = 20;
    const MAX_ROOM_HEIGHT: i32 = 50;

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH);
        let h = rng.range(MIN_ROOM_HEIGHT, MAX_ROOM_HEIGHT);
        let x = rng.range(1, map.width - w - 1);
        let y = rng.range(1, map.height - h - 1);

        let new_room = Rect::with_size(x, y, w, h);
        let mut quit = false;

        for other_rooms in map.rooms.iter() {
            if other_rooms.intersect(&new_room) {
                quit = true;
            }
        }

        if quit {
            continue;
        }

        map.build_room(new_room);
        if map.rooms.len() > 1 {
            let new_center = new_room.center();
            let prev_center = map.rooms[map.rooms.len() - 2].center();

            if rng.rand::<f32>() > 0.5 {
                map.build_hallway(prev_center, new_center);
            } else {
                map.build_hallway(new_center, prev_center);
            }
        }
    }

    map.set_blocked_tiles();

    let exit_room = map.rooms.len() - 1;
    let exit_x = rng.range(map.rooms[exit_room].x1, map.rooms[exit_room].x2);
    let exit_y = rng.range(map.rooms[exit_room].y1, map.rooms[exit_room].y2);
    map.level_exit = map.get_index(exit_x, exit_y);
    println!("{}", map.level_exit);

    map
}

pub fn build_level(ecs: &mut specs::World, width: i32, height: i32, depth: i32) -> Map {
    let mut map = {
        let mut rng = ecs.fetch_mut::<rltk::RandomNumberGenerator>();
        build_rogue_map(width, height, depth, &mut rng)
    };

    // we need to clone the list of rooms so that spawner can borrow the map
    let cloned_rooms = map.rooms.clone();
    let mut spawner = spawner::Spawner::new(ecs, &mut map, width);

    for room in cloned_rooms.iter() {
        let quality = depth;
        let mut spawn_ary = Vec::new();
        spawn_ary.push(
            spawner::build_mook as for<'r> fn(&'r mut specs::World, rltk::Point) -> specs::Entity,
        );
        spawn_ary.push(spawner::build_archer);
        spawner.build(
            &room,
            0 + quality / 2,
            2 + quality,
            vec![0.7, 0.3],
            spawn_ary,
        );

        let mut builder_ary = Vec::new();
        builder_ary.push(spawner::build_empty_barrel);

        spawner.build_with_quality(&room, 5, 10, depth, vec![1.0], builder_ary);
    }

    map
}
