mod pyramid;
mod console;

use console_gui::console_view::{View, Color};
use console_gui::console_view::point::Point;
use std::borrow::BorrowMut;

fn main() {
    let mut view = View::new(10, 20);
    view.draw_bg_line(Point(0, 0), Point(6, 6), Color(128, 128, 128, 128));
    view.draw_view();

    let mut input_sum = String::new();
    let mut input = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input);
        if input == "br" {
            break;
        }
        print!("{}\x1B[1;1", input);
        input_sum.push_str(&input);
        input_sum.push('\n');
    }

}

enum Vehicles {
    CAR(u8, u8),
    TRUCK(u8, u8),
    MOTORCYCLE(u8, u8)
}

