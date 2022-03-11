use super::quest::{Quest, QuestType};

pub struct QuestLog {
    pub entries: Vec<Quest>,
}

impl QuestLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_quest(&mut self, rng: &mut rltk::RandomNumberGenerator) {
        let area_info = crate::data::get_random_area(rng);
        let quest = Quest {
            quest_type: QuestType::Hunt,
            quest_targets: random_target(rng),
            area_name: area_info.name,
            map_builder_args: crate::map_builder::MapBuilderArgs {
                builder_type: area_info.map_type,
                height: 40,
                width: 40,
                depth: 1,
                map_color: area_info.color,
            },
            reward: 120,
            turn_limit: 300,
            completed: false,
            days_remaining: 3,
        };

        self.entries.push(quest);
    }

    pub fn advance_day(&mut self) {
        // remove all quests that have no days remaining
        self.entries.retain(|quest| quest.days_remaining > 1);

        // update the days on the remaining quests
        for quest in self.entries.iter_mut() {
            quest.days_remaining -= 1;
        }
    }
}

fn random_target(rng: &mut rltk::RandomNumberGenerator) -> Vec<String> {
    vec!["Legiana".to_string(), "Diablos".to_string()]
}
