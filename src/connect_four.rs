use std::io;
use crate::console_style::{
    yellow_color_text,
    red_color_text
};
use crate::common_function::end_game_or_start_new;

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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn get_transform(&self) -> (i32, i32) {
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

struct GridSize {
    rows: usize,
    cols: usize,
}
impl GridSize {
    fn new(rows: usize, cols: usize) -> GridSize {
        GridSize { rows, cols }
    }
}

struct GameState {
    grid: Vec<Vec<Token>>,
    grid_size: GridSize,
    player: Player,
}

impl GameState {
    fn new(rows: usize, cols: usize) -> GameState {
        GameState {
            grid: vec![vec![Token::Empty; cols]; rows],
            grid_size: GridSize::new(rows, cols),
            player: Player::Red,
        }
    }

    /// Return position if valid
    fn get_position(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row < self.grid_size.rows && col < self.grid_size.cols {
            Some((row, col))
        } else {
            None
        }
    }

    /// Return the next valid position in a given direction or None
    fn transform_position(
        &self,
        row: usize,
        col: usize,
        dir: &Direction,
    ) -> Option<(usize, usize)> {
        let (hor, ver) = dir.get_transform();
        let row = row.checked_add_signed(hor as isize)?;
        let col = col.checked_add_signed(ver as isize)?;
        self.get_position(row, col)
    }

    /// Get the token at a position or None if out of bounds
    fn get_token(&self, row: usize, col: usize) -> Option<&Token> {
        self.grid.get(row)?.get(col)
    }

    /// Calculate number token connected to last in
    fn direction_score(&self, mut row: usize, mut col: usize, dir: &Direction) -> usize {
        let Some(start_token) = self.get_token(row, col) else {return 0};

        match start_token {
            Token::Empty => return 0,
            _ => (),
        };

        let mut counter: usize = 0;
        loop {
            let Some((new_row, new_col)) = self.transform_position(row, col, dir) else {return counter};
            let token = self.get_token(new_row, new_col);
            match token {
                Some(token) => {
                    if start_token == token {
                        row = new_row;
                        col = new_col;
                        counter += 1;
                        continue;
                    } else {
                        return counter;
                    }
                }
                None => return counter,
            }
        }
    }

    fn token_score(&self, row: usize, col: usize) -> usize {
        let vertical: usize = [Direction::Up, Direction::Down]
            .iter()
            .map(|dir| self.direction_score(row, col, dir))
            .sum();
        let horizontal: usize = [Direction::Left, Direction::Right]
            .iter()
            .map(|dir| self.direction_score(row, col, dir))
            .sum();
        let diagonal: usize = [Direction::UpRight, Direction::DownLeft]
            .iter()
            .map(|dir| self.direction_score(row, col, dir))
            .sum();
        let reverse: usize = [Direction::UpLeft, Direction::DownRight]
            .iter()
            .map(|dir| self.direction_score(row, col, dir))
            .sum();

        println!("vertical {}", vertical);
        println!("horizontal {}", horizontal);
        println!("diagonal {}", diagonal);
        println!("reverse {}", reverse);
        
        let direction_sums = [vertical, horizontal, diagonal, reverse];
        let Some(s) = direction_sums.iter().max() else {return 0};

        *s
    }

    fn print_grid(&self) {
        for row in &self.grid {
            print!("{}", CELL_SEPARATOR);
            for token in row {
                match token {
                    Token::Red => red_color_text("O", true),
                    Token::Yellow => yellow_color_text("X", true),
                    Token::Empty => print!("{}", EMPTY_CELL),
                }
                print!("{}", CELL_SEPARATOR);
            }
            println!("");
            println!("{}", "-".repeat(self.grid_size.cols * 2 + 1));
        }
    }

    fn print_number(&self) {
        for i in 0..self.grid_size.cols {
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

    fn get_col_input(&self) -> usize {
        print!("It's ");
        match self.player {
            Player::Red => red_color_text("red", true),
            Player::Yellow => yellow_color_text("yellow", true),
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
                Ok(i) if i < self.grid_size.cols => break i,
                Ok(i) => println!("{} is greater than {} columns!", i, self.grid_size.cols),
                Err(..) => println!("{} is not a column number!", trimmed),
            };
        };

        col
    }

    fn insert_token(&mut self, col: usize) -> Option<(usize, usize)> {
        for row in (0..self.grid_size.rows).rev() {
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
        // if 3 tokens are connected to last in is a win
        self.token_score(row, col) >= 3
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
        let col = game_state.get_col_input();

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
            game_state.print_state();
            match game_state.player {
                Player::Red => red_color_text("red wins! :D", false),
                Player::Yellow => yellow_color_text("yellow wins! :D", false),
            }
            return end_game_or_start_new(game, name.to_string());
        }

        // todo: check tie (all slots taken but no winner)
        if game_state.check_tie() {
            todo!();
        }

        // switch player
        game_state.switch_player();
    }
}
