use crate::*;
use rltk::{Algorithm2D, Point};
use std::collections::HashMap;

const MAX_MONSTERS: i32 = 4;

fn room_table(_map_depth: i32) -> Vec<(String, f32)> {
    let mut spawn_ary = Vec::new();
    spawn_ary.push(("mook".to_string(), 0.7));
    spawn_ary.push(("archer".to_string(), 0.3));
    spawn_ary
}

fn build_from_name(ecs: &mut World, name: &String, index: usize) -> Option<Entity> {
    let point = { ecs.fetch::<Map>().index_to_point2d(index) };

    match name.as_ref() {
        "mook" => Some(build_mook(ecs, point)),
        "archer" => Some(build_archer(ecs, point)),
        _ => None,
    }
}

/// Fills a region with stuff!
pub fn spawn_region(ecs: &mut World, area: &[usize], map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    {
        let mut rng = ecs.fetch_mut::<rltk::RandomNumberGenerator>();
        let num_spawns = i32::min(
            areas.len() as i32,
            rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3,
        );

        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (rng.roll_dice(1, areas.len() as i32) - 1) as usize
            };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, roll(&spawn_table, &mut *rng));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for (map_idx, name) in spawn_points.iter() {
        //let point = map.index_to_point2d(*map_idx);
        let entity = build_from_name(ecs, name, *map_idx);
        // track_entity(ecs, &mut *map, entity, point);
        // spawn_entity(ecs, &spawn);

        // track the entity if we built one
        if let Some(entity) = entity {
            track_entity(ecs, entity, *map_idx);
        }
    }
}

pub fn track_entity(ecs: &mut World, entity: Entity, map_idx: usize) {
    let mut map = ecs.fetch_mut::<Map>();
    let multis = ecs.read_storage::<MultiTile>();
    map.track_creature(entity, map_idx, multis.get(entity));
}

fn roll(chance: &Vec<(String, f32)>, rng: &mut rltk::RandomNumberGenerator) -> String {
    let roll = rng.rand::<f32>();
    let mut cumul_prob = 0.0;

    for index in 0..chance.len() {
        cumul_prob += chance[index].1;

        if roll < cumul_prob {
            return chance[index].0.to_string();
        }
    }

    chance[0].0.to_string()
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
            range: 20,
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
            range: 10,
        })
        .with(BlocksTile)
        .with(AiState {
            status: Behavior::Wander,
            prev_path: None,
            path_step: 0,
        })
}

pub fn build_mook(ecs: &mut World, point: Point) -> Entity {
    let part_list = vec![
        MonsterPart {
            symbol_map: HashMap::from([
                (rltk::Point::new(-1, 0), rltk::to_cp437('│')),
                (rltk::Point::new(-1, 1), rltk::to_cp437('└')),
                (rltk::Point::new(0, 1), rltk::to_cp437('─')),
            ]),
            health: 1,
            max_health: 1,
        },
        MonsterPart {
            symbol_map: HashMap::from([
                (rltk::Point::new(0, -1), rltk::to_cp437('─')),
                (rltk::Point::new(1, -1), rltk::to_cp437('┐')),
                (rltk::Point::new(1, 0), rltk::to_cp437('│')),
            ]),
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
            bump_attack: AttackType::Punch,
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
            bump_attack: AttackType::Punch,
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
// #endregion

fn build_npc_base(ecs: &mut World, point: Point) -> EntityBuilder {
    ecs.create_entity()
        .with(Position {
            x: point.x,
            y: point.y,
        })
        .with(ViewableIndex { list_index: None })
        .with(Schedulable {
            current: 0,
            base: 24,
            delta: 4,
        })
        .with(Viewshed {
            visible: Vec::new(),
            dirty: true,
            range: 20,
        })
        .with(BlocksTile)
        .with(Health {
            current: 10,
            max: 10,
        })
}

pub fn build_npc_blacksmith(ecs: &mut World, point: Point) -> Entity {
    build_npc_base(ecs, point)
        .with(Renderable {
            symbol: rltk::to_cp437('@'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Blacksmith".to_string(),
            description: vec!["That's you!".to_string()],
            seen: false,
        })
        .with(Npc {
            npc_type: NpcType::Blacksmith,
        })
        .build()
}

pub fn build_npc_shopkeeper(ecs: &mut World, point: Point) -> Entity {
    build_npc_base(ecs, point)
        .with(Renderable {
            symbol: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Shopkeeper".to_string(),
            description: vec!["That's you!".to_string()],
            seen: false,
        })
        .with(Npc {
            npc_type: NpcType::Shopkeeper,
        })
        .build()
}

pub fn build_npc_handler(ecs: &mut World, point: Point) -> Entity {
    build_npc_base(ecs, point)
        .with(Renderable {
            symbol: rltk::to_cp437('@'),
            fg: RGB::named(rltk::BLUE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewable {
            name: "Handler".to_string(),
            description: vec!["That's you!".to_string()],
            seen: false,
        })
        .with(Npc {
            npc_type: NpcType::Handler,
        })
        .build()
}
