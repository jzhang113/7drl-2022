use crate::{Map, MoveIntent};
use rltk::Algorithm2D;
use specs::prelude::*;

pub enum Behavior {
    Sleep,
    Wander,
    Chase {
        target_point: rltk::Point,
    },
    Attack {
        attack: crate::AttackType,
        attack_loc: rltk::Point,
    },
    Flee,
}

enum NextIntent {
    None,
    Attack { intent: crate::AttackIntent },
    Move { intent: crate::MoveIntent },
}

pub struct AiSystem;

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, crate::CanActFlag>,
        ReadStorage<'a, crate::Position>,
        WriteStorage<'a, crate::MoveIntent>,
        WriteStorage<'a, crate::AttackIntent>,
        WriteStorage<'a, crate::AiState>,
        ReadStorage<'a, crate::Viewshed>,
        ReadStorage<'a, crate::Moveset>,
        ReadStorage<'a, crate::MultiTile>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, crate::Map>,
        WriteExpect<'a, crate::ParticleBuilder>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut can_act,
            positions,
            mut moves,
            mut attacks,
            mut states,
            viewsheds,
            movesets,
            multis,
            player,
            mut map,
            mut p_builder,
            mut rng,
        ) = data;
        let mut turn_done = Vec::new();
        let player_point = positions.get(*player).unwrap().as_point();

        for (ent, _turn, pos, state, viewshed, moveset, multi) in (
            &entities,
            &can_act,
            &positions,
            &mut states,
            &viewsheds,
            &movesets,
            (&multis).maybe(),
        )
            .join()
        {
            let action = self.next_step(
                ent,
                pos,
                state,
                viewshed,
                moveset,
                multi,
                player_point,
                &mut *map,
                &mut *p_builder,
                &mut *rng,
            );

            match action {
                NextIntent::Attack { intent } => {
                    attacks
                        .insert(ent, intent)
                        .expect("Failed to insert attack from AI");
                }
                NextIntent::Move { intent } => {
                    moves
                        .insert(ent, intent)
                        .expect("Failed to insert movement from AI");
                }
                NextIntent::None => {}
            }

            turn_done.push(ent);
        }

        for done in turn_done.iter() {
            can_act.remove(*done);
        }
    }
}

impl AiSystem {
    fn next_step(
        &mut self,
        ent: Entity,
        pos: &crate::Position,
        state: &mut crate::AiState,
        viewshed: &crate::Viewshed,
        moveset: &crate::Moveset,
        multi: Option<&crate::MultiTile>,
        player_point: rltk::Point,
        map: &mut crate::Map,
        _p_builder: &mut crate::ParticleBuilder,
        rng: &mut rltk::RandomNumberGenerator,
    ) -> NextIntent {
        let curr_index = map.get_index(pos.x, pos.y);

        loop {
            match state.status {
                Behavior::Sleep => {
                    // the do nothing state
                    // TODO: trigger wake up
                    return NextIntent::None;
                }
                Behavior::Wander => {
                    if can_see_target(viewshed, player_point) {
                        state.status = Behavior::Chase {
                            target_point: player_point,
                        };
                    } else {
                        // pick a random tile we can move to
                        let exits = map.get_available_exits_for(curr_index, ent, multi);
                        if exits.len() > 0 {
                            let exit_index = rng.range(0, exits.len());
                            let chosen_exit = exits[exit_index].0;
                            return NextIntent::Move {
                                intent: MoveIntent {
                                    loc: map.index_to_point2d(chosen_exit),
                                },
                            };
                        } else {
                            // TODO: help we're stuck
                            return NextIntent::None;
                        }
                    }
                }
                Behavior::Chase { target_point } => {
                    if can_see_target(viewshed, player_point) {
                        // track the player's current position
                        state.status = Behavior::Chase {
                            target_point: player_point,
                        };

                        // check if we have any attacks that can hit
                        let orig_point = pos.as_point();

                        let rolled_prob: f32 = rng.rand();
                        let mut cumul_prob: f32 = 0.0;
                        let mut attack_found = false;

                        // TODO: smarter attack selection
                        // this is fine when all of the attacks have similar attack ranges
                        // however, we might run into cases where we are in range to attack, but we decided to use an attack thats not valid
                        for (potential_attack, chance) in moveset.moves.iter() {
                            cumul_prob += chance;
                            if rolled_prob > cumul_prob {
                                continue;
                            }

                            if let Some(attack_loc) = crate::attack_type::is_attack_valid(
                                *potential_attack,
                                orig_point,
                                player_point,
                            ) {
                                state.status = Behavior::Attack {
                                    attack: *potential_attack,
                                    attack_loc: attack_loc,
                                };
                                attack_found = true;
                                break;
                            }
                        }

                        if !attack_found {
                            // if we can't hit, just move towards the player
                            let player_index = map.point2d_to_index(player_point);
                            let movement = move_towards(ent, map, curr_index, player_index, multi);

                            match movement {
                                None => {
                                    // we can't move towards the player for some reason, so give up chasing
                                    state.status = Behavior::Wander;
                                    return NextIntent::None;
                                }
                                Some(movement) => {
                                    return NextIntent::Move { intent: movement };
                                }
                            }
                        }
                    } else {
                        // we don't see the player, move to the last tracked point
                        let target_index = map.point2d_to_index(target_point);
                        let movement = move_towards(ent, map, curr_index, target_index, multi);

                        match movement {
                            None => {
                                // most likely reason we got here is because we reached the target point
                                // if we didn't see the player on the way, return to wandering
                                state.status = Behavior::Wander;
                                return NextIntent::None;
                            }
                            Some(movement) => {
                                return NextIntent::Move { intent: movement };
                            }
                        }
                    }
                }
                Behavior::Attack { attack, attack_loc } => {
                    let intent = crate::attack_type::get_attack_intent(attack, attack_loc, None);

                    if can_see_target(viewshed, player_point) {
                        state.status = Behavior::Chase {
                            target_point: player_point,
                        };
                    } else {
                        state.status = Behavior::Wander;
                    }

                    return NextIntent::Attack { intent };
                }
                Behavior::Flee => {
                    // TODO
                    return NextIntent::None;
                }
            }
        }
    }
}

fn can_see_target(viewshed: &crate::Viewshed, target: rltk::Point) -> bool {
    viewshed
        .visible
        .iter()
        .any(|pos| pos.x == target.x && pos.y == target.y)
}

fn move_towards(
    entity: Entity,
    map: &mut Map,
    curr_index: usize,
    target_index: usize,
    multi_component: Option<&crate::MultiTile>,
) -> Option<MoveIntent> {
    map.set_additional_args(entity, multi_component);
    let path = rltk::a_star_search(curr_index, target_index, &*map);

    if path.success && path.steps.len() > 1 {
        let next_pos = map.index_to_point2d(path.steps[1]);
        return Some(MoveIntent { loc: next_pos });
    } else {
        println!("No path exists!");
        return None;
    }
}
