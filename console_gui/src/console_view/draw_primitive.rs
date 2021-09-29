use crate::console_view::point::Point;

use super::{Color, View};

impl View {
    pub fn draw_line(&mut self, from: Point, to: Point, color: Color) {
        for x in from.0..to.0 {
            for y in from.1..to.1 {
                self.rows[y][x].bg_color = color.clone();
            }
        }
    }
}