#[macro_use]
extern crate lazy_static;

rltk::embedded_resource!(FONT, "../resources/Zilk-16x16.png");
rltk::embedded_resource!(ICONS, "../resources/custom_icons.png");

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod attack_type;
mod colors;
mod components;
mod gamelog;
mod gui;
mod map;
mod monster_part;
mod player;
mod range_type;
mod spawner;
mod sys_ai;
mod sys_attack;
mod sys_death;
mod sys_mapindex;
mod sys_movement;
mod sys_partbreak;
mod sys_particle;
mod sys_partmove;
mod sys_pickup;
mod sys_turn;
mod sys_visibility;

pub use attack_type::*;
pub use colors::*;
pub use components::*;
pub use map::{Map, TileType};
pub use monster_part::*;
pub use range_type::*;
pub use sys_ai::{Behavior, NextIntent};
pub use sys_particle::{ParticleBuilder, ParticleRequest};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    Targetting {
        attack_type: AttackType,
        ignore_targetting: bool,
    },
    ViewEnemy {
        index: u32,
    },
    ViewCard,
    Running,
    HitPause {
        remaining_time: f32,
    },
    GenerateMap,
    Dead,
}

pub struct State {
    ecs: World,
    tick: i32,
    cursor: rltk::Point,
    tab_targets: Vec<rltk::Point>,
    tab_index: usize,
    attack_modifier: Option<AttackType>,
}

impl State {
    fn register_components(&mut self) {
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<Player>();
        self.ecs.register::<Viewshed>();
        self.ecs.register::<CanActFlag>();
        self.ecs.register::<CanReactFlag>();
        self.ecs.register::<Schedulable>();
        self.ecs.register::<ParticleLifetime>();
        self.ecs.register::<BlocksTile>();
        self.ecs.register::<Viewable>();
        self.ecs.register::<ViewableIndex>();

        self.ecs.register::<Health>();
        self.ecs.register::<AttackIntent>();
        self.ecs.register::<MoveIntent>();
        self.ecs.register::<PartMoveIntent>();
        self.ecs.register::<Moveset>();

        self.ecs.register::<AttackInProgress>();
        self.ecs.register::<BlockAttack>();
        self.ecs.register::<AiState>();
        self.ecs.register::<Heal>();
        self.ecs.register::<Item>();
        self.ecs.register::<Openable>();

        self.ecs.register::<MultiTile>();
        self.ecs.register::<Facing>();
    }

    fn new_game(&mut self) {
        self.register_components();

        self.ecs.insert(RunState::Running);
        self.ecs.insert(sys_particle::ParticleBuilder::new());

        let rng = rltk::RandomNumberGenerator::new();
        self.ecs.insert(rng);

        let mut map = map::build_level(&mut self.ecs, gui::MAP_W, gui::MAP_H, 1);
        let player_pos = map.rooms[0].center();
        let player = spawner::build_player(&mut self.ecs, player_pos);
        map.track_creature(player, player_pos, None);

        self.ecs.insert(map);
        self.ecs.insert(player);

        let log = gamelog::GameLog {
            entries: vec!["Hello world!".to_string()],
        };
        self.ecs.insert(log);
    }

    fn run_systems(&mut self) -> RunState {
        self.tick += 1;

        sys_ai::AiSystem.run_now(&self.ecs);
        sys_turn::TurnSystem.run_now(&self.ecs);

        sys_movement::MovementSystem.run_now(&self.ecs);
        sys_attack::AttackSystem.run_now(&self.ecs);
        sys_partmove::PartMoveSystem.run_now(&self.ecs);
        sys_partbreak::PartBreakSystem.run_now(&self.ecs);

        // pickups happen after movement
        sys_pickup::PickupSystem.run_now(&self.ecs);

        // index needs to run after movement so blocked tiles are updated
        sys_mapindex::MapIndexSystem.run_now(&self.ecs);

        // death needs to run after attacks so bodies are cleaned up
        sys_death::DeathSystem.run_now(&self.ecs);

        sys_visibility::VisibilitySystem.run_now(&self.ecs);
        sys_particle::ParticleSpawnSystem.run_now(&self.ecs);

        self.ecs.maintain();

        RunState::Running
    }

    fn entities_need_cleanup(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();

        let mut to_delete = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            // Don't delete the player
            let p = player.get(entity);
            if let Some(_p) = p {
                should_delete = false;
            }

            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    fn change_level(&mut self) {
        // Delete entities that aren't the player or his/her equipment
        let to_delete = self.entities_need_cleanup();
        for target in to_delete {
            self.ecs
                .delete_entity(target)
                .expect("Unable to delete entity");
        }

        let curr_depth = {
            let map = self.ecs.fetch::<Map>();
            map.depth
        };

        let new_map = crate::map::build_level(
            &mut self.ecs,
            crate::gui::MAP_W,
            crate::gui::MAP_H,
            curr_depth + 1,
        );

        // update player position
        let player = self.ecs.fetch::<Entity>();
        let mut positions = self.ecs.write_storage::<Position>();
        let mut player_pos = positions
            .get_mut(*player)
            .expect("player didn't have a position");

        let new_player_pos = new_map.rooms[0].center();
        player_pos.x = new_player_pos.x;
        player_pos.y = new_player_pos.y;

        // replace map
        let mut map_writer = self.ecs.write_resource::<Map>();
        *map_writer = new_map;

        // Mark the player's visibility as dirty
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(*player);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // cleanup
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        sys_particle::cleanup_particles(&mut self.ecs, ctx);

        // draw map + gui
        gui::draw_map(&self.ecs, ctx);
        gui::draw_renderables(&self.ecs, ctx);
        gui::draw_sidebar(&self.ecs, ctx);

        gui::draw_blocked_tiles(&self.ecs, ctx);
        gui::draw_attacks_in_progress(&self.ecs, ctx);

        let mut next_status;
        let player_point;

        // wrapping to limit borrowed lifetimes
        {
            let player = self.ecs.fetch::<Entity>();
            let positions = self.ecs.read_storage::<Position>();
            let player_pos = positions
                .get(*player)
                .expect("player didn't have a position");
            player_point = rltk::Point::new(player_pos.x, player_pos.y);

            // get the current RunState
            next_status = *self.ecs.fetch::<RunState>();
        }

        match next_status {
            RunState::AwaitingInput => {
                gui::update_controls_text(&self.ecs, ctx, &next_status);
                next_status = player::player_input(self, ctx);

                if next_status == RunState::Running {
                    player::end_turn_cleanup(&mut self.ecs);
                }
            }
            RunState::Targetting {
                attack_type,
                ignore_targetting,
            } => {
                gui::update_controls_text(&self.ecs, ctx, &next_status);
                let range_type = crate::attack_type::get_attack_range(attack_type);
                let tiles_in_range = crate::range_type::resolve_range_at(&range_type, player_point);

                let result = player::ranged_target(self, ctx, tiles_in_range, ignore_targetting);
                match result.0 {
                    player::SelectionResult::Canceled => {
                        next_status = RunState::AwaitingInput;
                    }
                    player::SelectionResult::NoResponse => {}
                    player::SelectionResult::Selected => {
                        {
                            // we should generally have a target at this point
                            // if we don't have a point, assume its because we won't need one later
                            let target = result.1.unwrap_or(rltk::Point::zero());
                            let intent = crate::attack_type::get_attack_intent(
                                attack_type,
                                target,
                                self.attack_modifier,
                            );
                            let player = self.ecs.fetch::<Entity>();
                            let mut attacks = self.ecs.write_storage::<AttackIntent>();

                            attacks
                                .insert(*player, intent)
                                .expect("Failed to insert attack from Player");

                            self.attack_modifier = None;
                        }

                        next_status = RunState::Running;
                        player::end_turn_cleanup(&mut self.ecs);
                    }
                }
            }
            RunState::ViewEnemy { index } => {
                gui::update_controls_text(&self.ecs, ctx, &next_status);
                next_status = player::view_input(self, ctx, index);
            }
            RunState::ViewCard => {
                gui::update_controls_text(&self.ecs, ctx, &next_status);
                next_status = player::view_input(self, ctx, 0);
            }
            RunState::Running => {
                // uncomment while loop to skip rendering intermediate states
                while next_status == RunState::Running {
                    next_status = self.run_systems();

                    if next_status != RunState::Running {
                        break;
                    }

                    // std::thread::sleep(std::time::Duration::from_millis(100));
                    next_status = *self.ecs.fetch::<RunState>();
                }
            }
            RunState::HitPause { remaining_time } => {
                {
                    gui::update_controls_text(&self.ecs, ctx, &next_status);
                }

                sys_particle::ParticleSpawnSystem.run_now(&self.ecs);

                let new_time = remaining_time - ctx.frame_time_ms;
                if new_time < 0.0 {
                    next_status = RunState::Running;
                } else {
                    next_status = RunState::HitPause {
                        remaining_time: new_time,
                    }
                }
            }
            RunState::GenerateMap => {
                self.change_level();
                next_status = RunState::AwaitingInput;
            }
            RunState::Dead => {
                gui::update_controls_text(&self.ecs, ctx, &next_status);

                match ctx.key {
                    None => {}
                    Some(key) => {
                        if key == rltk::VirtualKeyCode::R {
                            let new_world = World::new();
                            self.ecs = new_world;
                            self.new_game();
                            next_status = RunState::Running;
                        }
                    }
                }
            }
        }

        let mut status_writer = self.ecs.write_resource::<RunState>();
        *status_writer = next_status;
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    rltk::link_resource!(FONT, "resources/Zilk-16x16.png");
    rltk::link_resource!(ICONS, "resources/custom_icons.png");

    let context = RltkBuilder::simple(gui::CONSOLE_WIDTH, gui::CONSOLE_HEIGHT)?
        .with_title("counterpuncher")
        .with_font("Zilk-16x16.png", 16, 16)
        .with_font("custom_icons.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .with_simple_console_no_bg(gui::CONSOLE_WIDTH, gui::CONSOLE_HEIGHT, "Zilk-16x16.png") // main layer
        .with_sparse_console_no_bg(gui::CONSOLE_WIDTH, gui::CONSOLE_HEIGHT, "custom_icons.png") // custom icons
        .with_sparse_console(gui::CONSOLE_WIDTH, gui::CONSOLE_HEIGHT, "Zilk-16x16.png") // control line
        .build()
        .expect("Failed to build console");

    let mut gs = State {
        ecs: World::new(),
        tick: 0,
        cursor: rltk::Point::zero(),
        tab_targets: Vec::new(),
        tab_index: 0,
        attack_modifier: None,
    };

    gs.new_game();

    rltk::main_loop(context, gs)
}
