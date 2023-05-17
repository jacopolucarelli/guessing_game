use std::io;
use crate::console_style::{
    yellow_color_text,
    green_color_text,
    red_color_text
};
use crate::common_function::end_game_or_start_new;

pub fn game(name: String) {
    println!("{}, guess the word!", name.trim());

    // eng word
    // let secret_word = random_word::gen();

    // ita word
    let secret_word = random_ita_word();
    let v: String = secret_word.chars().map(|_c| "_".to_string()).collect();
    type_letter(secret_word.to_string(), v, 10, name);
}

fn type_letter(secret_word: String, mut v: String, mut num_try: u32, name: String) {
    println!("{}", v);
    let underscore_index = v.find('_').unwrap_or(v.len());

    if  underscore_index == v.len() {
        green_color_text("You win :D", false);
        print!("You find the secret word ");
        green_color_text(&secret_word, false);
        return end_game_or_start_new(game, name);
    }

    if  num_try == 0 {
        red_color_text("You lose :(", false);
        print!("The secret word was ");
        red_color_text(&secret_word, false);
        return end_game_or_start_new(game, name);
    }

    let message = "You have # more attempts. Type a letter...".replace("#", num_try.to_string().as_str());
    yellow_color_text(&message, false);

    let mut letter = String::new();
    io::stdin()
    .read_line(&mut letter)
    .expect("Failed to read line");

    let mut index = 0;
    let mut find_letter = false;
    for c in secret_word.chars() { 
        if c.to_string().eq(&letter.trim().to_lowercase()) {
            v.replace_range(index..index+1, &letter.trim().to_lowercase());
            find_letter = true;
        }
        index += 1;
    }
    if !find_letter {
        num_try -= 1;
    }
    type_letter(secret_word, v, num_try, name);
}

fn random_ita_word() -> String {
    let response = reqwest::blocking::get("https://www.palabrasaleatorias.com/parole-casuali.php?fs=1&fs2=0&Submit=Nuova+parola",)
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("td>div").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    let mut word: String = String::new();
    titles
        .for_each(|item| word = item);
    word = word.trim().to_lowercase();
    return word;
}