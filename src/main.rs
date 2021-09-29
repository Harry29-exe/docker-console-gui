mod pyramid;
mod console;

use std::borrow::Borrow;
use console_gui::test;

use console_gui::console_view::{View, Color};
use console_gui::console_view::point::Point;

fn main() {
    let mut view = View::new(20, 20);
    view.draw_line(Point(0,0), Point(5,5), Color(128,128,128,128));
    view.draw_view();
}

enum Vehicles {
    CAR(u8, u8),
    TRUCK(u8, u8),
    MOTORCYCLE(u8, u8)
}

