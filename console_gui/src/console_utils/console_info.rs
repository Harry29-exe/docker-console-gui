
pub fn get_console_size() -> (u16, u16) {
    (0, 0)
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
