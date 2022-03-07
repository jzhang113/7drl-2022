use crate::*;
use rltk::{Point, RandomNumberGenerator, Rect};

pub struct Spawner<'a> {
    ecs: &'a mut World,
    map: &'a mut Map,
    map_width: i32,
}

impl<'a> Spawner<'a> {
    pub fn new(ecs: &'a mut World, map: &'a mut Map, map_width: i32) -> Self {
        Spawner {
            ecs,
            map,
            map_width,
        }
    }

    fn get_spawn_points(
        &mut self,
        room: &Rect,
        min: i32,
        max: i32,
        chance: Vec<f32>,
    ) -> Vec<(usize, i32, i32)> {
        let mut spawn_points = Vec::new();
        let mut rng = self.ecs.fetch_mut::<RandomNumberGenerator>();
        let spawn_count = rng.range(min, max);

        for _ in 0..spawn_count {
            let dx = rng.range(1, room.width());
            let dy = rng.range(1, room.height());
            let xpos = room.x1 + dx;
            let ypos = room.y1 + dy;
            let index = ((ypos * self.map_width) + xpos) as usize;

            // don't spawn over something else
            if !self.map.blocked_tiles[index] && index != self.map.level_exit {
                let roll = rng.rand::<f32>();
                let mut cumul_prob = 0.0;
                let mut builder_index = 0;

                for index in 0..chance.len() {
                    cumul_prob += chance[index];

                    if roll < cumul_prob {
                        builder_index = index;
                        break;
                    }
                }

                spawn_points.push((builder_index, xpos, ypos));
            }
        }

        spawn_points
    }

    fn track_entity(&mut self, entity: Entity, point: Point) {
        let multis = self.ecs.read_storage::<MultiTile>();
        self.map.track_creature(entity, point, multis.get(entity));
    }

    pub fn build(
        &mut self,
        room: &Rect,
        min: i32,
        max: i32,
        chance: Vec<f32>,
        builder: Vec<impl Fn(&mut World, Point) -> Entity>,
    ) {
        for (builder_index, xpos, ypos) in self.get_spawn_points(room, min, max, chance) {
            let point = Point::new(xpos, ypos);
            let entity = builder[builder_index](self.ecs, point);
            self.track_entity(entity, point);
        }
    }

    pub fn build_with_quality(
        &mut self,
        room: &Rect,
        min: i32,
        max: i32,
        quality: i32,
        chance: Vec<f32>,
        builder: Vec<impl Fn(&mut World, Point, i32) -> Entity>,
    ) {
        for (builder_index, xpos, ypos) in self.get_spawn_points(room, min, max, chance) {
            let point = Point::new(xpos, ypos);
            let entity = builder[builder_index](self.ecs, point, quality);
            self.track_entity(entity, point);
        }
    }
}

// #region Player
pub fn build_player(ecs: &mut World, point: Point) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: point.x,
            y: point.y,
        })
        .with(Renderable {
            symbol: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Player".to_string(),
            description: vec!["That's you!".to_string()],
            seen: false,
        })
        .with(ViewableIndex { list_index: None })
        .with(Player)
        .with(Schedulable {
            current: 0,
            base: 24,
            delta: 4,
        })
        .with(Viewshed {
            visible: Vec::new(),
            dirty: true,
            range: 8,
        })
        .with(CanReactFlag)
        //.with(BlocksTile)
        .with(Health {
            current: 10,
            max: 10,
        })
        .build()
}
// #endregion

// #region Enemies
pub fn build_enemy_base(ecs: &mut World) -> EntityBuilder {
    ecs.create_entity()
        .with(ViewableIndex { list_index: None })
        .with(Schedulable {
            current: 0,
            base: 24,
            delta: 4,
        })
        .with(Viewshed {
            visible: Vec::new(),
            dirty: true,
            range: 8,
        })
        .with(BlocksTile)
        .with(AiState {
            status: Behavior::Wander,
        })
}

pub fn build_mook(ecs: &mut World, point: Point) -> Entity {
    let mut part_hash = std::collections::HashMap::new();
    part_hash.insert(rltk::Point::new(1, 1), rltk::to_cp437('!'));
    part_hash.insert(rltk::Point::new(1, 2), rltk::to_cp437('!'));

    let mut part_hash_2 = std::collections::HashMap::new();
    part_hash_2.insert(rltk::Point::new(-1, 1), rltk::to_cp437('!'));
    part_hash_2.insert(rltk::Point::new(-1, 2), rltk::to_cp437('!'));

    let part_list = vec![
        MonsterPart {
            symbol_map: part_hash,
            health: 1,
            max_health: 1,
        },
        MonsterPart {
            symbol_map: part_hash_2,
            health: 1,
            max_health: 1,
        },
    ];

    build_enemy_base(ecs)
        .with(Position {
            x: point.x,
            y: point.y,
        })
        .with(Renderable {
            symbol: rltk::to_cp437('x'),
            fg: RGB::named(rltk::LIGHT_BLUE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Mook".to_string(),
            description: vec![
                "A lowly grunt,".to_string(),
                "unskilled, but".to_string(),
                "can still pack".to_string(),
                "a wallop".to_string(),
            ],
            seen: false,
        })
        .with(Health {
            current: 10,
            max: 10,
        })
        .with(Moveset {
            moves: vec![(AttackType::Sweep, 0.25), (AttackType::Punch, 0.75)],
        })
        .with(MultiTile {
            bounds: all_bounds(&part_list),
            part_list: part_list,
        })
        .with(Facing {
            direction: crate::Direction::N,
        })
        .build()
}

pub fn build_archer(ecs: &mut World, point: Point) -> Entity {
    build_enemy_base(ecs)
        .with(Position {
            x: point.x,
            y: point.y,
        })
        .with(Renderable {
            symbol: rltk::to_cp437('y'),
            fg: RGB::named(rltk::LIGHT_GREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Archer".to_string(),
            description: vec!["A grunt with a bow".to_string()],
            seen: false,
        })
        .with(Health { current: 2, max: 2 })
        .with(Moveset {
            moves: vec![(AttackType::Punch, 0.25), (AttackType::Ranged, 0.75)],
        })
        .build()
}
// #endregion

// #region Objects
fn barrel_builder(ecs: &mut World, point: Point) -> EntityBuilder {
    ecs.create_entity()
        .with(Position {
            x: point.x,
            y: point.y,
        })
        .with(Renderable {
            symbol: rltk::to_cp437('#'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Barrel".to_string(),
            description: vec![
                "A barrel, what".to_string(),
                "could be".to_string(),
                "inside?".to_string(),
            ],
            seen: false,
        })
        .with(BlocksTile)
        .with(Openable)
        .with(Health { current: 2, max: 2 })
}

pub fn build_empty_barrel(ecs: &mut World, point: Point, _quality: i32) -> Entity {
    barrel_builder(ecs, point).build()
}

pub fn _build_health_pickup(ecs: &mut World, point: Point, quality: i32) -> Entity {
    ecs.create_entity()
        .with(crate::Position {
            x: point.x,
            y: point.y,
        })
        .with(crate::Renderable {
            symbol: rltk::to_cp437('+'),
            fg: crate::health_color(),
            bg: crate::bg_color(),
        })
        .with(crate::Heal {
            amount: quality as u32,
        })
        .with(crate::Viewable {
            name: "health".to_string(),
            description: vec!["Packaged health, don't ask".to_string()],
            seen: false,
        })
        .build()
}
