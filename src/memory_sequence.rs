use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use rand::{distributions::Alphanumeric, Rng};
use std::io;

pub fn game(name: String) {

    println!("{}, remember the sequence!", name.trim());
    super::console_style::green_color_text("Press enter when you ready", false);
    
    let mut _ready = String::new();
    io::stdin()
        .read_line(&mut _ready)
        .expect("Failed to read line");
    let str = String::new();
    encrease_string(str, name);
}

fn encrease_string(mut str: String, name: String) {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
    str.push_str(&s);
    clear_console(&str);

    let mut sequence = String::new();
    
    println!("Type the sequence:");
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    if str.to_string().eq(&sequence.trim()) {
        encrease_string(str, name);
    } else {
        
        super::console_style::red_color_text("You lose :(", false);
        print!("Sequence was ");
        super::console_style::green_color_text(str.as_str(), true);
        print!(" but you type ");
        super::console_style::red_color_text(sequence.trim(), false);
        print!("You have memorized ");
        super::console_style::green_color_text((str.len()-1).to_string().as_str(), true);
        println!(" char sequence! :D");
        return super::common_function::end_game_or_start_new(game, name);
    }
}

fn clear_console(str: &str) {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    for i in (1..20).rev() {
        stdout.queue(cursor::SavePosition).unwrap();
        stdout.write_all(format!("{}: {}", i, str).as_bytes()).unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
    stdout.execute(cursor::Show).unwrap();
}