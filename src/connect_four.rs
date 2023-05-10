use std::io;

const EMPTY_CELL: &str = " ";
const CELL_SEPARATOR: &str = "|";

enum Player {
    Red,
    Yellow,
}
impl Player {
    fn get_token(&self) -> Token {
        match self {
            Player::Red => Token::Red,
            Player::Yellow => Token::Yellow,
        }
    }
}

#[derive(Clone)]
enum Token {
    Red,
    Yellow,
    Empty,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn get_transform(&self) -> (usize, usize) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}

struct GameState {
    grid: Vec<Vec<Token>>,
    rows: usize,
    cols: usize,
    player: Player,
}

impl GameState {
    fn new(rows: usize, cols: usize) -> GameState {
        GameState {
            grid: vec![vec![Token::Empty; cols]; rows],
            rows,
            cols,
            player: Player::Red,
        }
    }

    /// Return position if valid
    fn get_position(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row < self.rows && col < self.cols {
            Some((new_row, new_col))
        } else {
            None
        }
    }

    /// Return the next valid position in a given direction or None
    fn transform_position(&self, row: usize, col: usize, dir: Direction) -> Option<(usize, usize)> {
        let (h, v) = dir.get_transform();
        let (new_row, new_col) = (row+h, col+v);
        self.get_position(new_row, new_col)
    }

    /// Get the token at a position or None if out of bounds
    fn get_token(&self, row: usize, col: usize) -> Option<Token> {
        self.grid.get(row)?.get(col)
    }

    /// Calculate the token score in a single direction from a starting position
    fn direction_score(&self, row: usize, col: usize, dir: Direction) -> usize {
    }

    fn print_grid(&self) {
        for row in &self.grid {
            print!("{}", CELL_SEPARATOR);
            for token in row {
                match token {
                    Token::Red => super::console_style::red_color_text("O", true),
                    Token::Yellow => super::console_style::yellow_color_text("X", true),
                    Token::Empty => print!("{}", EMPTY_CELL),
                }
                print!("{}", CELL_SEPARATOR);
            }
            println!("");
            println!("{}", "-".repeat(self.cols * 2 + 1));
        }
    }

    fn print_number(&self) {
        for i in 0..self.cols {
            print!("{}{}", EMPTY_CELL, i);
        }
    }

    fn print_state(&self) {
        self.print_number();
        println!("");
        self.print_grid();
    }

    fn switch_player(&mut self) {
        self.player = match self.player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
    }

    fn get_col_input(&self, name: &String) -> usize {
        print!("It's ");
        match self.player {
            Player::Red => super::console_style::red_color_text("red", true),
            Player::Yellow => super::console_style::yellow_color_text("yellow", true),
        }
        println!(" turn");

        let col = loop {
            // println!("{}, submit a number from 0 to {}", name.trim(), self.cols);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let trimmed = input.trim();
            match trimmed.parse::<usize>() {
                Ok(i) if i < self.cols => break i,
                Ok(i) => println!("{} is greater than {} columns!", i, self.cols),
                Err(..) => println!("{} is not a column number!", trimmed),
            };
        };

        col
    }

    fn insert_token(&mut self, col: usize) -> Option<(usize, usize)> {
        for row in (0..self.rows).rev() {
            match self.grid[row][col] {
                Token::Empty => {
                    self.grid[row][col] = self.player.get_token();
                    return Some((row, col));
                }
                _ => continue,
            }
        }
        None
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        false
    }

    fn check_tie(&self) -> bool {
        false
    }
}

pub fn game(name: String) {
    let mut game_state = GameState::new(6, 7);
    // todo: print intro
    loop {
        // todo: print game state
        game_state.print_state();

        // ask for col input & validate loop (col exists, has room) -> usize
        let col = game_state.get_col_input(&name);

        // insert token, get row, col -> (usize, usize)
        let (row, col) = match game_state.insert_token(col) {
            Some((row, col)) => (row, col),
            None => {
                println!("Column {} is full!", col);
                continue;
            }
        };

        // todo: check win from coords
        if game_state.check_win(row, col) {
            todo!();
        }

        // todo: check tie (all slots taken but no winner)
        if game_state.check_tie() {
            todo!();
        }

        // switch player
        game_state.switch_player();
    }
}

fn check_end_game(
    v: &Vec<Vec<String>>,
    red_turn: bool,
    name: &String,
    row_index: usize,
    cell_index: usize,
) {
    if check_horizontal_line(v, red_turn)
        || check_vertical_line(v, red_turn, row_index, cell_index)
        || check_oblique_line(v, red_turn, row_index, cell_index)
    {
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

fn check_vertical_line(
    v: &Vec<Vec<String>>,
    red_turn: bool,
    row_index: usize,
    cell_index: usize,
) -> bool {
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
            return true;
        }
    }
    return false;
}

fn check_oblique_line(
    v: &Vec<Vec<String>>,
    red_turn: bool,
    row_index: usize,
    cell_index: usize,
) -> bool {
    let token = if red_turn { "🔴" } else { "🟡" };

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

fn check_oblique_line_left_up(
    v: &Vec<Vec<String>>,
    token: &str,
    row_index: usize,
    cell_index: usize,
    counter: usize,
) -> usize {
    {
        let row_index_signed: i32 = match row_index.try_into() {
            Ok(row_index) => row_index,
            Err(_) => panic!("couldn't fit in i32"),
        };

        let cell_index_signed: i32 = match cell_index.try_into() {
            Ok(cell_index) => cell_index,
            Err(_) => panic!("couldn't fit in i32"),
        };
        if row_index_signed - 1 < 0 || cell_index_signed - 1 < 0 || counter == 4 {
            let cell = &v[row_index][cell_index];
            if cell.eq(token) {
                return counter + 1;
            }
            return counter;
        }
    }

    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_left_up(v, token, row_index - 1, cell_index - 1, counter + 1);
    } else {
        return counter;
    }
}

fn check_oblique_line_left_down(
    v: &Vec<Vec<String>>,
    token: &str,
    row_index: usize,
    cell_index: usize,
    counter: usize,
) -> usize {
    if row_index + 1 > 6 || cell_index + 1 > 6 || counter == 4 {
        let cell = &v[row_index][cell_index];
        if cell.eq(token) {
            return counter + 1;
        }
        return counter;
    }

    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_left_down(v, token, row_index + 1, cell_index + 1, counter + 1);
    } else {
        return counter;
    }
}

fn check_oblique_line_right_up(
    v: &Vec<Vec<String>>,
    token: &str,
    row_index: usize,
    cell_index: usize,
    counter: usize,
) -> usize {
    {
        let row_index_signed: i32 = match row_index.try_into() {
            Ok(row_index) => row_index,
            Err(_) => panic!("couldn't fit in i32"),
        };

        if row_index_signed - 1 < 0 || cell_index + 1 > 6 || counter == 4 {
            let cell = &v[row_index][cell_index];
            if cell.eq(token) {
                return counter + 1;
            }
            return counter;
        }
    }

    let cell = &v[row_index][cell_index];
    if cell.eq(token) {
        return check_oblique_line_right_up(v, token, row_index - 1, cell_index + 1, counter + 1);
    } else {
        return counter;
    }
}

fn check_oblique_line_right_down(
    v: &Vec<Vec<String>>,
    token: &str,
    row_index: usize,
    cell_index: usize,
    counter: usize,
) -> usize {
    {
        let cell_index_signed: i32 = match cell_index.try_into() {
            Ok(cell_index) => cell_index,
            Err(_) => panic!("couldn't fit in i32"),
        };

        if cell_index_signed - 1 < 0 || row_index + 1 > 6 || counter == 4 {
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
        return check_oblique_line_right_down(v, token, row_index + 1, cell_index - 1, counter + 1);
    } else {
        return counter;
    }
}

fn end_game(v: &Vec<Vec<String>>, red_turn: bool, name: &String) {
    //print_state(&v);
    if red_turn {
        super::console_style::red_color_text("red wins! :D", false)
    } else {
        super::console_style::yellow_color_text("yellow wins! :D", false)
    }
    return super::common_function::end_game_or_start_new(game, name.to_string());
}
