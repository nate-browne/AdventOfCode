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

struct RopeNode {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

impl RopeNode {
    fn new() -> RopeNode {
        RopeNode {
            x: 0,
            y: 0,
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn move_head(&mut self, dr: Direction) {
        match dr {
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
        }
    }

    fn move_tail(&mut self, other_x: i32, other_y: i32) {
        let x_dist = other_x - self.x;
        let y_dist = other_y - self.y;

        if x_dist.abs() >= 2 || y_dist.abs() >= 2 {
            if x_dist > 0 {
                self.x += 1;
            } else if x_dist < 0 {
                self.x -= 1;
            }

            if y_dist > 0 {
                self.y += 1;
            } else if y_dist < 0 {
                self.y -= 1;
            }
        }
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
        let ln = line.context("Error occurred unwrapping String")?;
        let line_items: Vec<&str> = ln.split(' ').collect();
        res.push((line_items[0].to_string(), line_items[1].parse::<i32>().context("Error occurred parsing string as int")?));
    }
    Ok(res)
}

fn run_simulation(instructions: &Vec<(String, i32)>, num_nodes: i32) -> usize {
    let mut rope_snake = Vec::new();
    rope_snake.push(RopeNode::new());

    for _ in 0..(num_nodes - 1) {
        rope_snake.push(RopeNode::new());
    }

    for (dr, amt) in instructions {
        for _ in 0..*amt {
            for ind in 0..rope_snake.len() {
                if ind == 0 {
                    rope_snake[ind].move_head(Direction::letter_to_direction(dr));
                } else {
                    let other_x = rope_snake[ind - 1].x;
                    let other_y = rope_snake[ind - 1].y;
                    rope_snake[ind].move_tail(other_x, other_y);
                }
            }
        }
    }

    rope_snake[rope_snake.len() - 1].get_visited().len()
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

    let instructions = match parse_input_file(&conf.input_file) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Error parsing instructions file: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Part 1: {}", run_simulation(&instructions, 2));
    println!("Part 2: {}", run_simulation(&instructions, 10));

    ExitCode::SUCCESS
}
