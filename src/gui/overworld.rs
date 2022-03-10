use rltk::{Rltk, RGB};
use specs::World;

pub fn draw_missions(ecs: &World, ctx: &mut Rltk, selected_idx: usize) {
    ctx.print(1, 1, "Hello overmap");

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

    let mission_list = vec![
        "Back to Basics",
        "Learning to Ride",
        "The Basics of Capturing Monsters",
        "The Rampage Approaches",
    ];
    ctx.print(book_x + 5, book_y + 2, "Select Mission");

    for i in 0..mission_list.len() {
        let row = book_y + 4 + 2 * i;

        ctx.print(book_x + 1, row, mission_list[i]);
        ctx.print(book_x + 40, row, "Completed!");

        if i == selected_idx {
            ctx.set_active_console(0);
            for dx in 0..40 {
                ctx.set_bg(book_x + 1 + dx, row, RGB::named(rltk::YELLOW));
            }
            ctx.set_active_console(1);
        }
    }

    ctx.print(book_x + 1, book_y + 20, "Hunting Quest");
    ctx.print(book_x + 1, book_y + 22, "Hunting a Legiana");
    ctx.print(book_x + 1, book_y + 24, "Reward Money: 120z");
    ctx.print(book_x + 1, book_y + 26, "Time Limit: 300 turns");
}
