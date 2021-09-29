mod console_utils;
pub mod console_view;

// pub use crate::console_utils::console_utils;
pub mod console_gui {
}

pub fn test() {
    console_utils::console_info::clear();
    let size = console_utils::console_info::get_console_size();
    print!("{} {}", size.0, size.1);
}