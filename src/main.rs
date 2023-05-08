use std::io;
mod guessing_number;
mod hang_man;
mod memory_sequence;
mod common_function;
mod ascii_pics;
mod console_style;
mod force_four;

fn main() {
    start_game();
}

fn start_game() {
    common_function::print_string(ascii_pics::super_mario(), 50);
    let mut name = String::new();
    println!("Insert your name");
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
    common_function::menu(name);
}