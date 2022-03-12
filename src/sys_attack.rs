use crate::attack_type;
use rltk::Algorithm2D;
use specs::prelude::*;
use std::collections::HashMap;

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, crate::Map>,
        ReadStorage<'a, crate::Position>,
        WriteStorage<'a, crate::AttackIntent>,
        WriteStorage<'a, crate::AttackInProgress>,
        WriteStorage<'a, crate::Health>,
        WriteStorage<'a, crate::MultiTile>,
        WriteExpect<'a, crate::ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            positions,
            mut attacks,
            mut attacks_in_progress,
            mut healths,
            mut multis,
            mut p_builder,
        ) = data;
        let mut finished_attacks = Vec::new();

        for (ent, intent) in (&entities, &mut attacks).join() {
            if intent.delay > 0 {
                intent.delay -= 1;
                attacks_in_progress
                    .insert(ent, crate::AttackInProgress)
                    .expect("Failed to insert AttackInProgress flag");
                continue;
            }

            finished_attacks.push(ent);
            let trait_list = attack_type::get_attack_traits(intent.main);

            for att_trait in trait_list {
                match att_trait {
                    crate::AttackTrait::Knockback { amount: _ } => {
                        //
                    }
                    crate::AttackTrait::Damage { amount } => {
                        let targets = attack_type::each_attack_target(intent.main, intent.loc);
                        let mut ents_hit = HashMap::new();

                        for point in targets {
                            p_builder.make_bg_particle(point);
                            let point_index = map.point2d_to_index(point);
                            if let Some(aff_ent) = map.creature_map.get(&point_index) {
                                // avoid self damage
                                if *aff_ent == ent {
                                    continue;
                                }

                                let hit_locs = ents_hit.entry(aff_ent).or_insert(Vec::new());
                                hit_locs.push(point);
                            }
                        }

                        for (ent_hit, hit_locs) in ents_hit {
                            if let Some(mut aff_health) = healths.get_mut(*ent_hit) {
                                aff_health.current -= amount;

                                if let Some(aff_part) = multis.get_mut(*ent_hit) {
                                    if let Some(pos) = positions.get(*ent_hit) {
                                        for part in aff_part.part_list.iter_mut() {
                                            for part_pos in part.symbol_map.keys() {
                                                let adj_part_pos = pos.as_point() + *part_pos;

                                                if hit_locs.contains(&adj_part_pos) {
                                                    part.health -= 1;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                for pos in hit_locs {
                                    p_builder.make_hit_particle(pos);
                                }
                            }
                        }
                    }
                    crate::AttackTrait::Movement => {
                        //
                    }
                    crate::AttackTrait::Heal { amount: _ } => {
                        //
                    }
                    crate::AttackTrait::Equipment => {
                        // this is another marker
                    }
                }
            }
        }

        for done in finished_attacks.iter() {
            attacks.remove(*done);
            attacks_in_progress.remove(*done);
        }
    }
}
