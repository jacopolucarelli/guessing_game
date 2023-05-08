use std::io;

const EMPTY_CELL: &str = " ";
const CELL_SEPARATOR: &str = "|";

pub fn game(name: String) {
    let v = setup_grid();
    println!("");
    println!("{}, submit a number from 0 to 6", name.trim());
    next_move(v, true);
}

fn setup_grid() -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>>  = vec![];
    for _ in 0..7 {
        let mut row: Vec<String> = vec![];
        for _ in 0..7 {
            row.push(String::from(EMPTY_CELL));
        }
        v.push(row);
    }
    v
}

fn print_state(v: &Vec<Vec<String>>) {
    print_number();
    println!("");
    print_grid(v);
}

fn print_grid(v: &Vec<Vec<String>>) {
    for row in v {
        print!("|");
        for column in row {
            if column.eq("🟡") {
                super::console_style::yellow_color_text("X", true);
            }
            else if column.eq("🔴") {
                super::console_style::red_color_text("O", true);
            } else {
                print!("{}", column);
            }
            print!("{}", CELL_SEPARATOR);
        }
        println!("");
        println!("---------------");
    }
}

fn print_number() {
    for i in 0..7 {
        print!("{}{}", EMPTY_CELL, i);
    }
}

fn next_move(v: Vec<Vec<String>>, red_turn: bool) {
    print_state(&v);

    print!("It's ");
    if red_turn {
        super::console_style::red_color_text("red", true);
    } else {
        super::console_style::yellow_color_text("yellow", true);
    };
    println!(" turn");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    match input.trim() {   
        "0" => insert_token(v, 0, red_turn),
        "1" => insert_token(v, 1, red_turn),
        "2" => insert_token(v, 2, red_turn),
        "3" => insert_token(v, 3, red_turn),
        "4" => insert_token(v, 4, red_turn),
        "5" => insert_token(v, 5, red_turn),
        "6" => insert_token(v, 6, red_turn),
        _ => { 
            next_move(v, red_turn);
        },
    }
}

fn insert_token(mut v: Vec<Vec<String>>, position: usize, red_turn: bool) {
    let token = if red_turn {"🔴"} else {"🟡"};
    for i in (0..7).rev() {
        if v[i][position].trim().is_empty() {
            v[i][position] = String::from(token);
            break;
        } else {
            continue;
        }
    }
    next_move(v, !red_turn);
}