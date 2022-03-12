use super::consts::*;
use crate::quest::quest::Quest;
use crate::*;
use rltk::{Rltk, RGB};

pub fn draw_sidebar(ecs: &World, ctx: &mut Rltk, current_quest: &Option<Quest>) {
    let players = ecs.read_storage::<Player>();
    let healths = ecs.read_storage::<Health>();
    let stams = ecs.read_storage::<Stamina>();

    let rends = ecs.read_storage::<Renderable>();
    let in_progress = ecs.read_storage::<AttackInProgress>();
    let player = ecs.fetch::<Entity>();

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

    for (_, rend, stamina, health, attack) in
        (&players, &rends, &stams, &healths, (&in_progress).maybe()).join()
    {
        // change symbol color if attacking
        let symbol_color;
        if attack.is_some() {
            symbol_color = attack_highlight_color();
        } else {
            symbol_color = RGB::named(rltk::WHITE);
        }

        ctx.set(x, y, symbol_color, bg_color(), rend.symbol);
        ctx.set(
            x + 1,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(':'),
        );

        draw_resource_bar(
            ctx,
            health.current,
            health.max,
            x,
            y,
            hp_main_color(),
            hp_alt_color(),
        );

        draw_resource_bar(
            ctx,
            stamina.current,
            stamina.max,
            x,
            y + 2,
            stam_main_color(),
            stam_alt_color(),
        );
    }

    y += 4;

    if let Some(quest) = current_quest {
        ctx.print_color(
            x,
            y,
            crate::text_highlight_color(),
            crate::bg_color(),
            quest.get_name(),
        );
    } else {
        ctx.print_color(x, y, crate::text_failed_color(), crate::bg_color(), "None");
    }

    let invulns = ecs.read_storage::<Invulnerable>();
    if let Some(inv) = invulns.get(*player) {
        ctx.print(x, y + 2, "Invulnerable!");
    }

    super::tooltip::draw_tooltips(ecs, ctx);
}

fn draw_resource_bar(
    ctx: &mut Rltk,
    curr: i32,
    max: i32,
    x: i32,
    y: i32,
    main_color: RGB,
    alt_color: RGB,
) {
    let curr = std::cmp::max(0, curr);
    for i in 0..curr {
        ctx.set(x + i + 2, y, main_color, bg_color(), rltk::to_cp437('o'));
    }

    for i in curr..max {
        ctx.set(x + i + 2, y, alt_color, bg_color(), rltk::to_cp437('o'));
    }
}
