use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
    collections::HashSet,
};

use fs_err::File;
use anyhow::{Result, Context};

// Number of expected command line arguments
const EXPECTED_ARG_NUM: usize = 2;
const FILE_IDX: usize = 1;

struct Config {
    input_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != EXPECTED_ARG_NUM {
            return Err("number of passed in arguments incorrect.");
        }
        Ok(Config {
            input_file: args[FILE_IDX].clone(),
        })
    }
}

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NULL,
}

impl Direction {
    fn letter_to_direction(lt: &String) -> Direction {
        match lt.to_uppercase().as_str() {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            &_ => panic!("didn't input a valid direction!"),
        }
    }
}

struct Head {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Head {
    fn new() -> Head {
        Head {
            x: 0,
            y: 0,
            direction: Direction::NULL,
        }
    }

    fn reset_position(&mut self) {
        self.x = 0;
        self.y = 0;
        self.direction = Direction::NULL;
    }

    fn move_head(&mut self, dr: Direction) {
        self.direction = dr.clone();
        match dr {
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::NULL => (),
        }
    }
}

struct Tail {
    visited: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    max_distance: f64,
}

impl Tail {
    fn new() -> Tail {
        Tail {
            visited: HashSet::from([(0, 0)]),
            x: 0,
            y: 0,
            max_distance: 2_f64.sqrt(),
        }
    }

    fn _is_same_row(&self, h: &Head) -> bool {
        self.x != h.x && self.y == h.y
    }

    fn _is_same_col(&self, h: &Head) -> bool {
        self.x == h.x && self.y != h.y
    }

    fn move_tail(&mut self, h: &Head) {
        let distance = (((h.x - self.x).pow(2) + (h.y - self.y).pow(2)) as f64).sqrt();
        if distance > self.max_distance {
            if !self._is_same_col(h) && !self._is_same_row(h) {
                // if the head goes up or down, we need to match the column
                // if it goes left or right, we need to match the row
                match h.direction {
                    Direction::UP => {
                        self.x = h.x;
                        self.y += 1;
                    }
                    Direction::DOWN => {
                        self.x = h.x;
                        self.y -= 1;
                    }
                    Direction::LEFT => {
                        self.x -= 1;
                        self.y = h.y;
                    }
                    Direction::RIGHT => {
                        self.x += 1;
                        self.y = h.y;
                    }
                    Direction::NULL => (),
                }
            } else if self._is_same_row(h) {
                if h.x - self.x > 0 {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            } else if self._is_same_col(h) {
                if h.y - self.y > 0 {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
            self._add_to_visited();
        }
    }

    fn reset_position(&mut self) {
        self.x = 0;
        self.y = 0;
        self.visited.clear();
        self._add_to_visited();
    }

    fn get_visited(&self) -> &HashSet<(i32, i32)> {
        &self.visited
    }

    fn _add_to_visited(&mut self) {
        self.visited.insert((self.x, self.y));
    }
}

fn parse_input_file(input_file: &String) -> Result<Vec<(String, i32)>> {

    let infile = File::open(input_file)
        .context("Error occurred opening file.")?;
    let mut res = Vec::new();
    let reader = BufReader::new(infile);

    for line in reader.lines() {
        let ln = line?;
        let line_items: Vec<&str> = ln.split(' ').collect();
        res.push((line_items[0].to_string(), line_items[1].parse::<i32>()?));
    }
    Ok(res)
}

fn main() -> ExitCode {
    println!();

    let args: Vec<String> = env::args().collect();

    let conf = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing command line arguments: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut head = Head::new();
    let mut tail = Tail::new();
    println!("Initial head coordinates: ({}, {})", head.x, head.y);
    println!("Initial tail coordinates: ({}, {})", tail.x, tail.y);
    let instructions = match parse_input_file(&conf.input_file) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Error parsing instructions file: {e}");
            return ExitCode::FAILURE;
        }
    };

    for (dr, amt) in instructions {
        for _ in 0..amt {
            head.move_head(Direction::letter_to_direction(&dr));
            tail.move_tail(&head);
        }
    }
    println!("Final head coordinates: ({}, {})", head.x, head.y);
    println!("Final tail coordinates: ({}, {})", tail.x, tail.y);
    println!("Part 1: {}", tail.get_visited().len());

    ExitCode::SUCCESS
}
