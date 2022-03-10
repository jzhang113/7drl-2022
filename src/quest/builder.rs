use crate::data::AreaInfo;

pub struct Quest {
    pub name: String,
    pub quest_type: String,
    pub quest_targets: Vec<String>,
    pub area_name: String,
    pub map: crate::Map,
    pub reward: i32,
    pub turn_limit: i32,
    pub completed: bool,
    pub days_remaining: i32,
}

pub struct QuestBuilder;

impl QuestBuilder {
    // pub fn build(rng: &mut rltk::RandomNumberGenerator) -> Quest {}

    pub fn build_area(rng: &mut rltk::RandomNumberGenerator) -> AreaInfo {
        crate::data::get_random_area(rng)
    }
}
