pub mod draw_primitive;
pub mod point;

pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Clone for Color {
    fn clone(&self) -> Self {
        Color(self.0, self.1, self.2, self.3)
    }
}

impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 &&
            self.1 == other.1 &&
            self.2 == other.2 &&
            self.3 == other.3
    }
}

pub struct Px {
    pub bg_color: Color,
    pub color: Color,
    pub sign: char
}

impl Px {
    pub fn new(bg: Color, color: Color, sign: char) -> Self {
        Px {
            bg_color: bg,
            color: color,
            sign: sign
        }
    }
}

impl Clone for Px {
    fn clone(&self) -> Self {
        Px {
            bg_color: self.bg_color.clone(),
            color: self.color.clone(),
            sign: self.sign
        }
    }
}

impl PartialEq<Self> for Px {
    fn eq(&self, other: &Self) -> bool {
        self.bg_color == other.bg_color &&
            self.color == other.color &&
            self.sign == other.sign
    }
}


pub struct View {
    rows: Vec<Vec<Px>>,
    pub height: usize,
    pub width: usize
}


impl View {

    pub fn new(height: usize, width: usize) -> View {
        let mut rows = Vec::with_capacity( height);
        for i in 0..height {
            rows.push(Vec::with_capacity(width));
            for _j in 0..width {
                rows[i].push(Px {
                    bg_color: Color(0, 0, 0, 0),
                    color: Color(0,0,0,0),
                    sign: ' '
                });
            }
        }
        View {
            rows: rows,
            height,
            width
        }
    }

    pub fn get_px_at(&self, x: usize, y: usize) -> &Px {
        return &self.rows[x][y]
    }

    pub fn set_px_at(&mut self, x: usize, y: usize, value: Px) {
        self.rows[y][x] = value;
    }

    pub fn draw_view(self) {
        let mut last_px: &Px;
        let mut current_px: &Px;
        let mut string_po_print: String;
        for y in 0..self.height {
            string_po_print = String::from("\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m");
             last_px = &Px {
                bg_color: Color(0, 0, 0, 0),
                color: Color(0,0,0,0),
                sign: ' '
            };

            for x in 0..self.width {
                current_px = &self.rows[y][x];
                if current_px.bg_color == last_px.bg_color {
                    string_po_print.push(current_px.sign);
                } else {
                    string_po_print.push_str("\x1b[48;2;");
                    string_po_print.push_str(&current_px.bg_color.0.to_string());
                    string_po_print.push_str(";");
                    string_po_print.push_str(&current_px.bg_color.1.to_string());
                    string_po_print.push_str(";");
                    string_po_print.push_str(&current_px.bg_color.2.to_string());
                    string_po_print.push('m');
                    string_po_print.push(current_px.sign);
                }

                last_px = &current_px;
            }
            print!("{}\x1b[38;0m\x1b[48;0m\n", string_po_print);
        }
        // print!("\x1b[38;2;0m");
        // print!("\x1b[48;2;0m");

        print!("\x1b[0m");
        print!("\x1b[0m");
        print!("\n");
    }
}
