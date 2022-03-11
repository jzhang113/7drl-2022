use super::consts::*;
use crate::*;

pub fn update_controls_text(ecs: &World, ctx: &mut Rltk, status: &RunState) {
    ctx.set_active_console(3);

    // don't clear the previous line in hitpause
    match *status {
        RunState::HitPause { .. } => {}
        _ => ctx.cls(),
    };

    let x = 0;
    let y = CONSOLE_HEIGHT - 1;
    let icon_color = text_highlight_color();
    let bg_color = bg_color();
    let inactive_color = text_inactive_color();

    let is_reaction = {
        let can_act = ecs.read_storage::<CanActFlag>();
        let player = ecs.fetch::<Entity>();
        match can_act.get(*player) {
            None => false,
            Some(player_can_act) => player_can_act.is_reaction,
        }
    };

    if is_reaction {
        ctx.print(CONSOLE_WIDTH - 6, y, "REACT");
    } else {
        ctx.print_color(CONSOLE_WIDTH - 5, y, inactive_color, bg_color, "MAIN");
    }

    match *status {
        RunState::AwaitingInput => {
            // movement controls
            if is_reaction {
                draw_movement_controls(ctx, x, y, inactive_color, bg_color, true);
            } else {
                draw_movement_controls(ctx, x, y, icon_color, bg_color, false);
            }

            // examine
            let view_section_x = 13;
            ctx.print_color(view_section_x, y, icon_color, bg_color, "v");
            ctx.print(view_section_x + 1, y, "iew map");

            // space bar
            let space_section_x = 25;
            let space_action_str;
            if is_reaction {
                space_action_str = "block";
            } else {
                space_action_str = "draw card";
            }

            ctx.print_color(space_section_x, y, icon_color, bg_color, "[SPACE]");
            ctx.print(space_section_x + 8, y, space_action_str);

            // card section
            let card_section_x = 45;
            ctx.print_color(card_section_x, y, icon_color, bg_color, "[1-7]");
            ctx.print(card_section_x + 6, y, "use card");
        }
        RunState::Targetting {
            attack_type: _,
            ignore_targetting,
        } => {
            // movement controls
            if ignore_targetting {
                draw_movement_controls(ctx, x, y, inactive_color, bg_color, true);
            } else {
                draw_movement_controls(ctx, x, y, icon_color, bg_color, false);
            }

            // examine
            let view_section_x = 13;
            ctx.print_color(view_section_x, y, icon_color, bg_color, "v");
            ctx.print(view_section_x + 1, y, "iew card");

            // space bar
            let space_section_x = 25;
            ctx.print_color(space_section_x, y, icon_color, bg_color, "[SPACE]");
            ctx.print(space_section_x + 8, y, "confirm");

            // escape
            let escape_section_x = 45;
            ctx.print_color(escape_section_x, y, icon_color, bg_color, "[ESC]");
            ctx.print(escape_section_x + 6, y, "cancel");

            // tab target
            let tab_section_x = 60;
            if ignore_targetting {
                ctx.print_color(tab_section_x, y, inactive_color, bg_color, "[TAB]");
                ctx.print_color(
                    tab_section_x + 6,
                    y,
                    inactive_color,
                    bg_color,
                    "next target",
                );
            } else {
                ctx.print_color(tab_section_x, y, icon_color, bg_color, "[TAB]");
                ctx.print(tab_section_x + 6, y, "next target");
            }
        }
        RunState::ViewEnemy { .. } => {
            // movement controls
            draw_movement_controls(ctx, x, y, icon_color, bg_color, false);

            // escape
            let escape_section_x = 13;
            ctx.print_color(escape_section_x, y, icon_color, bg_color, "[ESC]");
            ctx.print(escape_section_x + 6, y, "cancel");
        }
        RunState::Dead { success } => {
            // restart
            ctx.print_color(x, y, icon_color, bg_color, "r");
            ctx.print(x + 1, y, "eturn to base");

            if success {
                ctx.print_color(
                    CONSOLE_WIDTH - 14,
                    y,
                    text_success_color(),
                    bg_color,
                    "QUEST COMPLETE",
                );
            } else {
                ctx.print_color(
                    CONSOLE_WIDTH - 12,
                    y,
                    text_failed_color(),
                    bg_color,
                    "QUEST FAILED",
                );
            }
        }
        RunState::HitPause { .. } => {
            ctx.print_color(CONSOLE_WIDTH - 6, y, inactive_color, bg_color, " WAIT");
        }
        _ => {}
    }

    ctx.set_active_console(1);
}

fn draw_movement_controls(ctx: &mut Rltk, x: i32, y: i32, fg: RGB, bg: RGB, inactive: bool) {
    ctx.set(x + 1, y, fg, bg, 27);
    ctx.set(x + 2, y, fg, bg, 25);
    ctx.set(x + 3, y, fg, bg, 24);
    ctx.set(x + 4, y, fg, bg, 26);

    if inactive {
        ctx.print_color(x + 6, y, fg, bg, "move");
    } else {
        ctx.print(x + 6, y, "move");
    }
}
