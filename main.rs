#![allow(dead_code)]

use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const DEAD_CHR: char = '.';
const ALIVE_CHR: char = '#';

struct Game {
    width:     usize,
    height:    usize,
    board:     Vec<bool>,
    backboard: Vec<bool>,
}

impl Game {
    fn new(name: &str) -> Self {
        let lines: Vec<String> = fs::read_to_string(name)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let h = lines.len();
        let w = lines[0].len();
        let mut board: Vec<bool> = Vec::with_capacity(w * h);
        for line in lines.iter() {
            for ch in line.chars() {
                let val = if ch == DEAD_CHR {
                    false
                } else if ch == ALIVE_CHR {
                    true
                } else {
                    panic!("invalid char at input file: {}", ch);
                };
                board.push(val);
            }
        }
        let backboard = board.clone();
        Game {
            width: w,
            height: h,
            board,
            backboard,
        }
    }

    fn run(&mut self) {
        loop {
            self.print_board();
            self.update();
            self.flip_board();
            sleep(Duration::from_millis(100));
        }
    }

    fn update(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let cur = self.at(col, row);
                let nbors = self.alive_nbors(col, row);
                let next = if cur {
                    (2..=3).contains(&nbors)
                } else {
                    nbors == 3
                };
                self.set_backboard_at(col, row, next);
            }
        }
    }

    fn flip_board(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let val = self.backboard_at(col, row);
                self.set_board_at(col, row, val);
            }
        }
    }

    fn print_board(&self) {
        clear();
        for row in 0..self.height {
            let mut line = String::new();
            for col in 0..self.width {
                let char = if self.at(col, row) {
                    "#"
                } else {
                    " "
                };
                line.push_str(char);
            }
            println!("{}", line);
        }
    }

    fn print_backboard(&self) {
        clear();
        for row in 0..self.height {
            let mut line = String::new();
            for col in 0..self.width {
                let char = if self.backboard_at(col, row) {
                    "#"
                } else {
                    "."
                };
                line.push_str(char);
            }
            println!("{}", line);
        }
    }

    fn backboard_at(&self, i: usize, j: usize) -> bool {
        self.backboard[j * self.width + i]
    }
    
    fn at(&self, i: usize, j: usize) -> bool {
        self.board[j * self.width + i]
    }

    fn set_board_at(&mut self, i: usize, j: usize, val: bool) {
        let idx = j * self.width + i;
        self.board[idx] = val;
    }

    fn set_backboard_at(&mut self, i: usize, j: usize, val: bool) {
        let idx = j * self.width + i;
        self.backboard[idx] = val;
    }

    fn alive_nbors(&self, i: usize, j: usize) -> usize {
        let mut alive = 0;
        let h = self.height as i32; 
        let w = self.width as i32; 
        let i = i as i32;
        let j = j as i32;
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let ny = (((j + dy) % h) + h) % h;
                let nx = (((i + dx) % w) + w) % w;
                let nbor = self.at(nx as usize, ny as usize);
                if nbor {
                    alive += 1;
                }
            }
        }
        alive
    }
}

fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("missing file input name");
    }
    let mut g = Game::new(&args[1]);
    // println!("W: {} | H: {} ", g.width, g.height);
    g.run();

}

