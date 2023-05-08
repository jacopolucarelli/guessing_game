use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn game(name: String) {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}, guess the number!", name.trim());
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => super::console_style::red_color_text("Too small!", false),
            Ordering::Greater => super::console_style::red_color_text("Too big!", false),
            Ordering::Equal => {
                super::console_style::green_color_text("You win :D", false);
                break;
            },
        }
    }
    return super::common_function::end_game_or_start_new(game, name);
}