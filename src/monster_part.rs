use std::collections::HashMap;

pub struct MonsterPart {
    pub symbol_map: HashMap<rltk::Point, rltk::FontCharType>,
    pub health: i32,
    pub max_health: i32,
}

impl MonsterPart {}
