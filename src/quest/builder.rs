pub struct Quest {
    pub quest_type: QuestType,
    pub quest_targets: Vec<String>,
    pub area_name: String,
    pub map_builder: Box<dyn crate::MapBuilder>,
    pub reward: i32,
    pub turn_limit: i32,
    pub completed: bool,
    pub days_remaining: i32,
}

impl Quest {
    pub fn get_name(&self) -> String {
        let mut name = "Hunt ".to_owned();
        name.push_str(&self.quest_targets.join(", "));
        name
    }
}

#[derive(Copy, Clone)]
pub enum QuestType {
    Hunt,
    Gather,
}

pub fn quest_type_name(quest_type: QuestType) -> String {
    match quest_type {
        QuestType::Hunt => "Hunting Quest".to_string(),
        QuestType::Gather => "Gathering Quest".to_string(),
    }
}

pub fn build(rng: &mut rltk::RandomNumberGenerator) -> Quest {
    let area_info = crate::data::get_random_area(rng);

    Quest {
        quest_type: QuestType::Hunt,
        quest_targets: random_target(rng),
        area_name: area_info.name,
        map_builder: crate::map_builder::with_builder(area_info.map_type, 40, 40, 1),
        reward: 120,
        turn_limit: 300,
        completed: false,
        days_remaining: 3,
    }
}

fn random_target(rng: &mut rltk::RandomNumberGenerator) -> Vec<String> {
    vec!["Legiana".to_string(), "Diablos".to_string()]
}
