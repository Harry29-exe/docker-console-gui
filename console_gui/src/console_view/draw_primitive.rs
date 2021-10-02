use crate::console_view::point::Point;

use super::{Color, View};

impl View {
    pub fn draw_bg_line(&mut self, from: Point, to: Point, color: Color) {
        let mut y: usize;
        let ab = calc_straigth_coefficients(&from, &to);
        for x in from.0 .. to.0 {
            y = (ab.0 * x as isize + ab.1) as usize;
            self.rows[y][x].bg_color = color.clone();
        }
    }


    pub fn draw_line(&mut self, from: Point, to: Point, color: Color, sign: char) {
        let mut y: usize;
        let ab = calc_straigth_coefficients(&from, &to);

        for x in from.0..to.0 {
            y = (ab.0 * x as isize  + ab.1) as usize;
            self.rows[y][x].color = color.clone();
            self.rows[y][x].sign = sign;
        }
    }

    pub fn draw_bg_rect(&mut self, from: Point, to: Point, color: Color) {
        for x in from.0 .. to.0 {
            for y in from.1 .. to.1 {
                self.rows[y][x].bg_color = color.clone();
            }
        }
    }
}

fn calc_straigth_coefficients(from: &Point, to: &Point) -> (isize, isize) {
    let a = (from.0 as isize - to.0 as isize) / (from.1 as isize - to.1 as isize);

    (a, from.1 as isize - a * from.0 as isize)
}