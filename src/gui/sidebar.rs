use super::consts::*;
use crate::*;
use rltk::{Rltk, RGB};

pub fn draw_sidebar(ecs: &World, ctx: &mut Rltk) {
    let healths = ecs.read_storage::<Health>();
    let rends = ecs.read_storage::<Renderable>();
    let mut viewables = ecs.write_storage::<ViewableIndex>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let positions = ecs.read_storage::<Position>();
    let in_progress = ecs.read_storage::<AttackInProgress>();

    let player = ecs.fetch::<Entity>();
    let player_view = viewsheds
        .get(*player)
        .expect("Player didn't have a viewshed");

    ctx.draw_box(
        SIDE_X,
        SIDE_Y,
        SIDE_W,
        SIDE_H + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let x = SIDE_X + 1;
    let mut y = SIDE_Y + 1;
    let mut index = 0;

    for (rend, mut view, pos, health, attack) in (
        &rends,
        &mut viewables,
        &positions,
        &healths,
        (&in_progress).maybe(),
    )
        .join()
    {
        if !player_view
            .visible
            .iter()
            .any(|view_pos| view_pos.x == pos.x && view_pos.y == pos.y)
        {
            continue;
        }

        view.list_index = Some(index);

        if index <= 5 {
            // change symbol color if attacking
            let symbol_color;
            if attack.is_some() {
                symbol_color = attack_highlight_color();
            } else {
                symbol_color = RGB::named(rltk::WHITE);
            }

            ctx.set(x, y, symbol_color, RGB::named(rltk::BLACK), rend.symbol);
            ctx.set(
                x + 1,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437(':'),
            );

            let curr_hp = std::cmp::max(0, health.current);

            for i in 0..curr_hp {
                ctx.set(
                    x + i + 2,
                    y,
                    hp_main_color(),
                    bg_color(),
                    rltk::to_cp437('o'),
                );
            }

            for i in curr_hp..health.max {
                ctx.set(
                    x + i + 2,
                    y,
                    hp_alt_color(),
                    bg_color(),
                    rltk::to_cp437('o'),
                );
            }
        }

        y += 2;
        index += 1;

        // TODO: what to do with excess?
    }

    ctx.draw_box(
        0,
        50,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let log = ecs.fetch::<gamelog::GameLog>();
    for (line, message) in log.entries.iter().rev().take(5).enumerate() {
        ctx.print(2, 50 + line + 1, message);
    }

    ctx.print(74, 1, format!("{} fps", ctx.fps));
    super::tooltip::draw_tooltips(ecs, ctx);
}
