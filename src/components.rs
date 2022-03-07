use rltk::{Point, RGB};
use specs::prelude::*;
use specs::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn as_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

#[derive(Component)]
pub struct Renderable {
    pub symbol: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible: Vec<Point>,
    pub dirty: bool,
    pub range: i32,
}

#[derive(Component)]
pub struct CanActFlag {
    pub is_reaction: bool,
    pub reaction_target: Option<Entity>,
}

#[derive(Component)]
pub struct CanReactFlag;

#[derive(Component)]
pub struct Schedulable {
    pub current: i32,
    pub base: i32,
    pub delta: i32,
}

#[derive(Component)]
pub struct ParticleLifetime {
    pub base: f32,
    pub remaining: f32,
    pub should_fade: bool,
}

#[derive(Component)]
pub struct BlocksTile;

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Copy, Clone)]
pub struct AttackIntent {
    pub main: crate::AttackType,
    pub modifier: Option<crate::AttackType>,
    pub loc: Point,
    pub delay: i32,
}

#[derive(Component)]
pub struct MoveIntent {
    pub loc: rltk::Point,
}

#[derive(Component)]
pub struct Moveset {
    pub moves: Vec<(crate::AttackType, f32)>,
}

#[derive(Component)]
pub struct Viewable {
    pub name: String,
    pub description: Vec<String>,
    pub seen: bool,
}

#[derive(Component)]
pub struct ViewableIndex {
    pub list_index: Option<u32>,
}

#[derive(Component)]
pub struct AttackInProgress;

#[derive(Component)]
pub struct BlockAttack {
    pub block_amount: u32,
}

#[derive(Component)]
pub struct AiState {
    pub status: crate::Behavior,
}

#[derive(Component)]
pub struct Heal {
    pub amount: u32,
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Openable;

pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Component)]
pub struct MultiTile {
    pub part_list: Vec<crate::MonsterPart>,
    pub facing: Direction,
    pub bounds: rltk::Rect,
}
