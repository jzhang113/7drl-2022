use super::{AttackIntent, Health, Map, ParticleBuilder, Position};
use crate::move_type;
use rltk::Algorithm2D;
use specs::prelude::*;

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, AttackIntent>,
        WriteStorage<'a, Health>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player, map, positions, mut attacks, mut healths, mut p_builder) = data;

        for (ent, intent) in (&entities, &attacks).join() {
            let trait_list = move_type::get_attack_traits(&intent.main);

            for att_trait in trait_list {
                match att_trait {
                    crate::AttackTrait::Knockback { amount } => {
                        //
                    }
                    crate::AttackTrait::Damage { amount } => {
                        println!("attack was executed");

                        let targets =
                            crate::move_type::each_attack_target(&intent.main, intent.loc);

                        for point in targets {
                            p_builder.make_particle(crate::ParticleRequest {
                                color: rltk::RGB::named(rltk::RED),
                                lifetime: 1000.0,
                                position: point,
                                symbol: rltk::to_cp437('!'),
                            });

                            let point_index = map.point2d_to_index(point);
                            if let Some(aff_ent) = map.creature_map.get(&point_index) {
                                if let Some(mut aff_health) = healths.get_mut(*aff_ent) {
                                    aff_health.current -= amount;
                                }
                            }
                        }
                    }
                    crate::AttackTrait::Movement => {
                        //
                    }
                    crate::AttackTrait::Heal { amount } => {
                        //
                    }
                    crate::AttackTrait::Equipment => {
                        // this is another marker
                    }
                }
            }
        }

        attacks.clear();
    }
}
