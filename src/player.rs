use crate::*;
use rltk::{Point, Rltk, VirtualKeyCode};

fn try_move_player(ecs: &mut World, dx: i32, dy: i32) -> RunState {
    use std::cmp::{max, min};
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut movements = ecs.write_storage::<MoveIntent>();
    let mut attacks = ecs.write_storage::<AttackIntent>();
    let mut healths = ecs.write_storage::<Health>();
    let openables = ecs.read_storage::<Openable>();
    let map = ecs.fetch::<Map>();
    let player = ecs.fetch::<Entity>();

    for (_player, pos) in (&players, &mut positions).join() {
        let new_x = min(map.width, max(0, pos.x + dx));
        let new_y = min(map.height, max(0, pos.y + dy));
        let dest_index = map.get_index(new_x, new_y);

        if dest_index == map.level_exit {
            return RunState::GenerateMap;
        }

        if !map.blocked_tiles[dest_index] {
            let new_move = MoveIntent {
                loc: Point::new(new_x, new_y),
            };
            movements
                .insert(*player, new_move)
                .expect("Failed to insert new movement from player");

            return RunState::Running;
        } else if map.tiles[dest_index] != crate::TileType::Wall {
            if let Some(dest_ent) = map.creature_map.get(&dest_index) {
                if let Some(_) = openables.get(*dest_ent) {
                    if let Some(health) = healths.get_mut(*dest_ent) {
                        // will be cleaned up by sys_death
                        health.current = 0;
                    }

                    return RunState::Running;
                } else {
                    let attack = crate::attack_type::get_attack_intent(
                        AttackType::Punch,
                        Point::new(new_x, new_y),
                        None,
                    );

                    attacks
                        .insert(*player, attack)
                        .expect("Failed to insert new attack from player");

                    return RunState::Running;
                }
            }

            return RunState::AwaitingInput;
        }
    }

    RunState::AwaitingInput
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let (is_reaction, target) = {
        let can_act = gs.ecs.read_storage::<super::CanActFlag>();
        let player = gs.ecs.fetch::<Entity>();
        let player_can_act = can_act
            .get(*player)
            .expect("player_input called, but it is not your turn");

        (player_can_act.is_reaction, player_can_act.reaction_target)
    };

    handle_keys(gs, ctx, is_reaction, target)
}

pub fn end_turn_cleanup(ecs: &mut World) {
    // remove can act flag
    // let player = ecs.fetch::<Entity>();
    let mut can_act = ecs.write_storage::<super::CanActFlag>();
    // let mut can_react = ecs.write_storage::<super::CanReactFlag>();

    // let is_reaction = {
    //     let can_act = ecs.read_storage::<super::CanActFlag>();
    //     let player = ecs.fetch::<Entity>();
    //     can_act
    //         .get(*player)
    //         .expect("player_input called, but it is not your turn")
    //         .is_reaction
    // };

    // if is_reaction {
    //     can_react.remove(*player);
    // } else {
    //     can_react
    //         .insert(*player, super::CanReactFlag {})
    //         .expect("Failed to insert CanReactFlag");
    // }

    can_act.clear();
}

fn handle_keys(
    gs: &mut State,
    ctx: &mut Rltk,
    is_reaction: bool,
    reaction_target: Option<Entity>,
) -> RunState {
    match ctx.key {
        None => RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                if is_reaction {
                    return RunState::AwaitingInput;
                } else {
                    return try_move_player(&mut gs.ecs, -1, 0);
                }
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                if is_reaction {
                    return RunState::AwaitingInput;
                } else {
                    return try_move_player(&mut gs.ecs, 1, 0);
                }
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                if is_reaction {
                    return RunState::AwaitingInput;
                } else {
                    return try_move_player(&mut gs.ecs, 0, -1);
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                if is_reaction {
                    return RunState::AwaitingInput;
                } else {
                    return try_move_player(&mut gs.ecs, 0, 1);
                }
            }
            VirtualKeyCode::Period => {
                return RunState::Running;
            }
            VirtualKeyCode::V => RunState::ViewEnemy { index: 0 },
            _ => RunState::AwaitingInput,
        },
    }
}

pub enum SelectionResult {
    Selected,
    Canceled,
    NoResponse,
}

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut Rltk,
    tiles_in_range: Vec<Point>,
    ignore_targetting: bool,
) -> (SelectionResult, Option<Point>) {
    let players = gs.ecs.read_storage::<Player>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    let mut valid_target = false;

    if ignore_targetting {
        ctx.print_color(
            crate::gui::MAP_X,
            crate::gui::MAP_Y - 1,
            crate::header_message_color(),
            crate::bg_color(),
            "Confirm use",
        );
    } else {
        ctx.set_active_console(0);

        // Highlight available target cells
        let mut available_cells = Vec::new();
        for (_player, viewshed) in (&players, &viewsheds).join() {
            // We have a viewshed
            for idx in viewshed.visible.iter() {
                if tiles_in_range.contains(idx) {
                    ctx.set_bg(
                        crate::gui::MAP_X + idx.x,
                        crate::gui::MAP_Y + idx.y,
                        crate::tiles_in_range_color(),
                    );
                    available_cells.push(idx);
                }
            }
        }

        // Draw cursor
        valid_target = available_cells
            .iter()
            .any(|pos| pos.x == gs.cursor.x && pos.y == gs.cursor.y);

        let cursor_color;
        if valid_target {
            cursor_color = crate::valid_cursor_color();
        } else {
            cursor_color = crate::invalid_cursor_color();
        }
        ctx.set_bg(
            crate::gui::MAP_X + gs.cursor.x,
            crate::gui::MAP_Y + gs.cursor.y,
            cursor_color,
        );
        ctx.set_active_console(1);

        if valid_target {
            ctx.print_color(
                crate::gui::MAP_X,
                crate::gui::MAP_Y - 1,
                crate::header_message_color(),
                crate::bg_color(),
                "Select Target",
            );
        } else {
            ctx.print_color(
                crate::gui::MAP_X,
                crate::gui::MAP_Y - 1,
                crate::header_err_color(),
                crate::bg_color(),
                "Invalid Target",
            );
        }
    }

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape => return (SelectionResult::Canceled, None),
            VirtualKeyCode::Space | VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                if valid_target {
                    return (
                        SelectionResult::Selected,
                        Some(Point::new(gs.cursor.x, gs.cursor.y)),
                    );
                } else if ignore_targetting {
                    return (SelectionResult::Selected, None);
                } else {
                    return (SelectionResult::Canceled, None);
                }
            }
            VirtualKeyCode::Tab => {
                let length = gs.tab_targets.len();

                if length > 0 {
                    gs.tab_index += 1;
                    gs.cursor = gs.tab_targets[gs.tab_index % length];
                }
            }
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                gs.cursor.x -= 1;
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                gs.cursor.x += 1;
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                gs.cursor.y -= 1;
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                gs.cursor.y += 1;
            }
            // TODO: placeholder
            VirtualKeyCode::V => return (SelectionResult::Canceled, None),
            _ => {}
        },
    };

    (SelectionResult::NoResponse, None)
}

pub fn view_input(gs: &mut State, ctx: &mut Rltk, index: u32) -> RunState {
    let entities = gs.ecs.entities();
    let v_indexes = gs.ecs.read_storage::<ViewableIndex>();
    let viewables = gs.ecs.read_storage::<Viewable>();

    let mut new_index = index;
    let mut max_index = 0;

    for (ent, viewables, v_index) in (&entities, &viewables, &v_indexes).join() {
        if let Some(list_index) = v_index.list_index {
            max_index = std::cmp::max(list_index, max_index);

            if list_index == index {
                crate::gui::draw_viewable_info(&gs.ecs, ctx, &ent, index);
            }
        }
    }

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape => return RunState::AwaitingInput,
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                if new_index > 0 {
                    new_index -= 1;
                } else {
                    new_index += max_index;
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                new_index += 1;
            }
            _ => {}
        },
    }

    RunState::ViewEnemy {
        index: new_index % (max_index + 1),
    }
}
