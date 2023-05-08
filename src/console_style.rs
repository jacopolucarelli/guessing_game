use console::style;

// black
// red
// green
// yellow
// blue
// magenta
// cyan
// white
// gray
// redBright
// greenBright
// yellowBright
// blueBright
// magentaBright
// cyanBright
// whiteBright

pub fn yellow_color_text(text: &str, is_inline: bool) {
    if is_inline {
        print!("{}", style(text).yellow());
    } else {
        println!("{}", style(text).yellow());
    }
}

pub fn red_color_text(text: &str, is_inline: bool) {
    if is_inline {
        print!("{}", style(text).red());
    } else {
        println!("{}", style(text).red());
    }
}

pub fn green_color_text(text: &str, is_inline: bool) {
    if is_inline {
        print!("{}", style(text).green());
    } else {
        println!("{}", style(text).green());
    }
}