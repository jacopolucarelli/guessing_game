use std::{thread, time};
use std::io;

pub fn menu(name: String) {
    let mut game_index = String::new();

    println!("{}, wich game you wanna play?", name.trim());
    println!("1 Guessing Number");
    println!("2 Hang Man");
    println!("3 Memory Sequence");
    println!("4 Connect Four");
    println!("* Quit");

    io::stdin()
        .read_line(&mut game_index)
        .expect("Failed to read line");

    match game_index.trim() {   
        "1" => return super::guessing_number::game(name),
        "2" => return super::hang_man::game(name),
        "3" => return super::memory_sequence::game(name),
        "4" => return super::connect_four::game(name),
        _ => { 
            print_string(super::ascii_pics::meme(), 50);
            thread::sleep(time::Duration::from_millis(150));
        },
    }
}

pub fn end_game_or_start_new<F: FnOnce(String)>(f: F, name: String) {
    super::console_style::yellow_color_text("Type NEW for a new game or something else to go back to menu", false);
    let mut command = String::new();
    io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
    if "NEW".eq(&command.trim().to_uppercase()) {
            return f(String::from("Once again"));
    } else {
        super::common_function::menu(name);
    }
}

pub fn print_string(str: String, sleep_print: u64) {
    for l in str.lines() {
        super::console_style::yellow_color_text(l, false);
        thread::sleep(time::Duration::from_millis(sleep_print));
    }
}

/*
pub fn print_file(file_name: &str, sleep_print: u64) {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                thread::sleep(time::Duration::from_millis(sleep_print));
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file= File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/