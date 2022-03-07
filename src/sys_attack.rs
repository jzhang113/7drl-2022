use crate::attack_type;
use rltk::Algorithm2D;
use specs::prelude::*;

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, crate::Map>,
        WriteStorage<'a, crate::AttackIntent>,
        WriteStorage<'a, crate::AttackInProgress>,
        WriteStorage<'a, crate::Health>,
        WriteExpect<'a, crate::ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, map, mut attacks, mut attacks_in_progress, mut healths, mut p_builder) =
            data;
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

                        for point in targets {
                            let point_index = map.point2d_to_index(point);
                            if let Some(aff_ent) = map.creature_map.get(&point_index) {
                                if let Some(mut aff_health) = healths.get_mut(*aff_ent) {
                                    aff_health.current -= amount;

                                    p_builder.make_particle(crate::ParticleRequest {
                                        color: rltk::RGB::named(rltk::RED),
                                        lifetime: 300.0,
                                        position: point,
                                        symbol: rltk::to_cp437('!'),
                                    });
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
