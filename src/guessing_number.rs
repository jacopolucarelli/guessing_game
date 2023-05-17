use std::io;
use rand::Rng;
use std::cmp::Ordering;
use crate::console_style::{
    green_color_text,
    red_color_text
};
use crate::common_function::end_game_or_start_new;

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
            Ordering::Less => red_color_text("Too small!", false),
            Ordering::Greater => red_color_text("Too big!", false),
            Ordering::Equal => {
                green_color_text("You win :D", false);
                break;
            },
        }
    }
    return end_game_or_start_new(game, name);
}