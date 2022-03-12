#[derive(PartialEq, Clone, Default)]
pub struct SpawnInfo {
    pub major_monsters: Vec<String>,
    pub minor_monsters: Vec<String>,
    pub resources: Vec<String>,
}

pub fn generate_spawn_info(rng: &rltk::RandomNumberGenerator) -> SpawnInfo {
    SpawnInfo {
        major_monsters: vec!["mook".to_string(), "archer".to_string()],
        minor_monsters: vec![],
        resources: vec![],
    }
}
