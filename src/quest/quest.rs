#[derive(Clone, PartialEq)]
pub struct Quest {
    pub quest_type: QuestType,
    pub quest_targets: Vec<String>,
    pub area_name: String,
    pub map_builder_args: crate::map_builder::MapBuilderArgs,
    pub reward: u32,
    pub turn_limit: u32,
    pub completed: bool,
    pub days_remaining: u8,
}

impl Quest {
    pub fn get_name(&self) -> String {
        let mut name = "Hunt ".to_owned();
        name.push_str(&self.quest_targets.join(", "));
        name
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum QuestType {
    Hunt,
    Gather,
    Urgent,
}

pub fn quest_type_name(quest_type: QuestType) -> String {
    match quest_type {
        QuestType::Hunt => "Hunting Quest".to_string(),
        QuestType::Gather => "Gathering Quest".to_string(),
        QuestType::Urgent => "Urgent Quest".to_string(),
    }
}
