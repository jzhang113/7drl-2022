use crate::quest::*;
use rltk::{Rltk, RGB};

pub fn draw_missions(
    ctx: &mut Rltk,
    quest_log: &log::QuestLog,
    current_quest: &Option<quest::Quest>,
    selected_idx: usize,
) {
    let book_x = 5;
    let book_y = 5;
    let book_page_w = 30;
    let book_page_h = 40;

    ctx.draw_box(
        book_x,
        book_y,
        book_page_w,
        book_page_h,
        RGB::named(rltk::GREY),
        RGB::named(rltk::BLACK),
    );
    ctx.draw_box(
        book_x + book_page_w,
        book_y,
        book_page_w,
        book_page_h,
        RGB::named(rltk::GREY),
        RGB::named(rltk::BLACK),
    );

    ctx.print(book_x + 5, book_y + 2, "Select Mission");

    for (i, quest) in quest_log.entries.iter().enumerate() {
        let row = book_y + 4 + 2 * i;

        ctx.print(book_x + 1, row, quest.get_name());

        ctx.print(book_x + 30, row, format!("{} days", quest.days_remaining));

        if quest.completed {
            ctx.print(book_x + 40, row, "Completed!");
        }

        if current_quest.as_ref().map_or(false, |q| q == quest) {
            ctx.print(book_x + 40, row, "Assigned!");
        }

        if i == selected_idx {
            ctx.set_active_console(0);
            for dx in 0..40 {
                ctx.set_bg(book_x + 1 + dx, row, RGB::named(rltk::YELLOW));
            }
            ctx.set_active_console(1);
        }
    }

    if selected_idx < quest_log.entries.len() {
        let quest = &quest_log.entries[selected_idx];
        ctx.print(
            book_x + 1,
            book_y + 20,
            quest::quest_type_name(quest.quest_type),
        );
        ctx.print(book_x + 1, book_y + 22, quest.get_name());
        ctx.print(
            book_x + 1,
            book_y + 24,
            format!("Reward Money: {}z", quest.reward),
        );
        ctx.print(
            book_x + 1,
            book_y + 26,
            format!("Time Limit: {} turns", quest.turn_limit),
        );
        ctx.print(book_x + 1, book_y + 28, quest.area_name.clone());
    }
}
