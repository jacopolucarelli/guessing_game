use std::{io};

const EMPTY_CELL: &str = " ";
const CELL_SEPARATOR: &str = "|";

pub fn game(name: String) {
    let v = setup_grid();
    println!("");
    println!("{}, submit a number from 0 to 6", name.trim());
    print_state(&v);
    next_move(v, true, name);
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

fn next_move(v: Vec<Vec<String>>, red_turn: bool, name:String) {
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
        "0" => insert_token(v, 0, red_turn, name),
        "1" => insert_token(v, 1, red_turn, name),
        "2" => insert_token(v, 2, red_turn, name),
        "3" => insert_token(v, 3, red_turn, name),
        "4" => insert_token(v, 4, red_turn, name),
        "5" => insert_token(v, 5, red_turn, name),
        "6" => insert_token(v, 6, red_turn, name),
        _ => { 
            next_move(v, red_turn, name);
        },
    }
}

fn insert_token(mut v: Vec<Vec<String>>, cell_index: usize, red_turn: bool, name:String) {
    let token = if red_turn {"🔴"} else {"🟡"};
    for row_index in (0..7).rev() {
        if v[row_index][cell_index].trim().is_empty() {
            v[row_index][cell_index] = String::from(token);
            check_end_game(&v, red_turn, &name, row_index, cell_index);
            break;
        } else {
            continue;
        }
    }
    print_state(&v);
    next_move(v, !red_turn, name);
}

fn check_end_game(v: &Vec<Vec<String>>, red_turn: bool, name: &String, row_index: usize, cell_index: usize) {
    if check_horizontal_line(v, red_turn) || 
            check_vertical_line(v, red_turn, row_index, cell_index) || 
            check_oblique_line(v, red_turn, row_index, cell_index) {
        end_game(v, red_turn, &name);
    }
}

fn check_horizontal_line(v: &Vec<Vec<String>>, red_turn: bool) -> bool {
    let mut counter = 0;
    for row in v {
        for cell in row {
            if counter == 4 {
                return true;
            }
            if red_turn && cell.eq("🔴") {
                counter += 1;
            } else if !red_turn && cell.eq("🟡") {
                counter += 1;
            } else {
                counter = 0;
            }
        }
    }
    return false;
}

fn check_vertical_line(v: &Vec<Vec<String>>, red_turn: bool, row_index: usize, cell_index: usize) -> bool {
    // La row più bassa della griglia ha indice 6: se l'ultimo token inserito ha indice maggiore di 3 non posso avere una vincita verticale
    if row_index > 3 {
        return false;
    }
    let mut counter = 0;
    // Parto dalla row dell'ultimo token inserito e scendo verso il basso (per scendere devo aumentare la row)
    for i in row_index..7 {
        let cell = &v[i][cell_index];
        if red_turn && cell.eq("🔴") {
            counter += 1;
        } else if !red_turn && cell.eq("🟡") {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == 4 {
            return true
        }
    }
    return false;
}

fn check_oblique_line(v: &Vec<Vec<String>>, red_turn: bool, row_index: usize, cell_index: usize) -> bool {
    
    let token = if red_turn {"🔴"} else {"🟡"};
    
    let mut counter = 0; //La cella in cui mi trovo la conto qui
    counter += check_oblique_line_left_up(v, token, row_index, cell_index, 0);
    counter += check_oblique_line_left_down(v, token, row_index, cell_index, 0);
    if counter == 5 {
        return true;
    }

    counter = 0;
    counter += check_oblique_line_right_up(v, token, row_index, cell_index, 0);
    counter += check_oblique_line_right_down(v, token, row_index, cell_index, 0);

    return counter == 5;
}

fn check_oblique_line_left_up(v: &Vec<Vec<String>>, token: &str, row_index: usize, cell_index: usize, counter: usize) -> usize {
    {
        let row_index_signed: i32 = match row_index.try_into() {
            Ok(row_index) => row_index,
            Err(_) => panic!("couldn't fit in i32"),
        };
    
        let cell_index_signed: i32 = match cell_index.try_into() {
            Ok(cell_index) => cell_index,
            Err(_) => panic!("couldn't fit in i32"),
        };
        if row_index_signed-1 < 0 || cell_index_signed-1 < 0 || counter == 4 {
            let cell = &v[row_index][cell_index];
            if cell.eq(token) {
                return counter + 1;
            }
            return counter;
        }
    }
    
    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_left_up(v, token, row_index-1, cell_index-1, counter+1); 
    } else {
        return counter;
    }
}

fn check_oblique_line_left_down(v: &Vec<Vec<String>>, token: &str, row_index: usize, cell_index: usize, counter: usize) -> usize {
    if row_index+1 > 6 || cell_index+1 > 6 || counter == 4 {
        let cell = &v[row_index][cell_index];
        if cell.eq(token) {
            return counter + 1;
        }
        return counter;
    }
    
    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_left_down(v, token, row_index+1, cell_index+1, counter+1); 
    } else {
        println!("check_oblique_line_left_down counter B{}", counter);
        return counter;
    }
}

fn check_oblique_line_right_up(v: &Vec<Vec<String>>, token: &str, row_index: usize, cell_index: usize, counter: usize) -> usize {
    {
        let row_index_signed: i32 = match row_index.try_into() {
            Ok(row_index) => row_index,
            Err(_) => panic!("couldn't fit in i32"),
        };
        
        if row_index_signed-1 < 0 || cell_index+1 > 6 || counter == 4 {
            let cell = &v[row_index][cell_index];
            if cell.eq(token) {
                return counter + 1;
            }
            return counter;
        }
    }
    
    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_right_up(v, token, row_index-1, cell_index+1, counter+1); 
    } else {
        return counter;
    }
}

fn check_oblique_line_right_down(v: &Vec<Vec<String>>, token: &str, row_index: usize, cell_index: usize, counter: usize) -> usize {
    {
        let cell_index_signed: i32 = match cell_index.try_into() {
            Ok(cell_index) => cell_index,
            Err(_) => panic!("couldn't fit in i32"),
        };
        
        if cell_index_signed-1 < 0 || row_index+1 > 6 || counter == 4 {
            let cell = &v[row_index][cell_index];
            if cell.eq(token) {
                return counter + 1;
            } else {
                return counter;
            }
        }
    }
    
    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_right_down(v, token, row_index+1, cell_index-1, counter+1); 
    } else {
       return counter;
    }
}

fn end_game(v: &Vec<Vec<String>>, red_turn: bool, name: &String) {
    print_state(&v);
    if red_turn {
        super::console_style::red_color_text("red wins! :D", false)
    } else {
        super::console_style::yellow_color_text("yellow wins! :D", false)
    }
    return super::common_function::end_game_or_start_new(game, name.to_string());
}