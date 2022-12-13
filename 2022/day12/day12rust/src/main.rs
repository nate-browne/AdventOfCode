use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
    collections::{HashSet, HashMap, VecDeque},
    convert::TryInto,
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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn point_in_bounds(point: &Point, grid: &Vec<Vec<char>>) -> bool {
    (0 <= point.x && point.x < (grid.len() as i32)) && (0 <= point.y && point.y < (grid[0].len() as i32))
}

fn letter_to_height(ltr: &char) -> i32 {
    let alpha = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut alpha_map = vec![];
    for (ind, ch) in alpha.enumerate() {
        alpha_map.push((ch, ind as i32));
    }
    let len = alpha_map.len();
    alpha_map.push(('S', 0));
    alpha_map.push(('E', 25));

    let arr_map: [(char, i32); 28] = match alpha_map.try_into() {
        Ok(am) => am,
        Err(_) => panic!("expected vector of length 28, received length {len}"),
    };

    let mp = HashMap::from(arr_map);
    mp[ltr]
}

fn create_grid(input_file: &String) -> Result<(Vec<Vec<char>>, Point, Point)> {
    let mut result = vec![];
    let infile = File::open(input_file)
        .context("Error occurred opening file")?;
    let reader = BufReader::new(infile);

    for line in reader.lines() {
        let ln = line.context("Error occurred unwrapping string")?;
        let char_vec: Vec<char> = ln.chars().collect();
        result.push(char_vec);
    }

    let mut start_p = Point::new(0, 0);
    let mut end_p = Point::new(0, 0);
    for x in 0..result.len() {
        for y in 0..result[0].len() {
            if result[x][y] == 'S' {
                start_p = Point::new(x as i32, y as i32);
            }
            if result[x][y] == 'E' {
                end_p = Point::new(x as i32, y as i32);
            }
        }
    }
    Ok((result, start_p, end_p))
}

fn bfs(grid: &Vec<Vec<char>>, start: &Point, end: &Point) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), 0));

    while queue.len() > 0 {
        let (current, dist) = match queue.pop_front() {
            Some((c, d)) => (c, d),
            None => panic!("Queue shouldn't be empty!"),
        };

        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);
        let current_height = grid[current.x as usize][current.y as usize];

        if current == *end {
            return dist;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nxt = Point::new(dx + current.x, dy + current.y);
            if point_in_bounds(&nxt, grid) {
                let new_height = grid[nxt.x as usize][nxt.y as usize];
                if letter_to_height(&new_height) - letter_to_height(&current_height) <= 1 {
                    queue.push_back((nxt.clone(), dist + 1));
                }
            }
        }
    }
    1_000_000 // no path exists
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

    let (grid, start_p, end_p) = match create_grid(&conf.input_file) {
        Ok((g, s, e)) => (g, s, e),
        Err(e) => {
            eprintln!("Error parsing file: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Part 1: {}", bfs(&grid, &start_p, &end_p));

    let mut starting_points = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if letter_to_height(&grid[x][y]) == 0 {
                starting_points.push(Point::new(x as i32, y as i32));
            }
        }
    }

    let mut part2_distances: Vec<i32> = starting_points.iter().map(|val| bfs(&grid, val, &end_p)).collect();
    part2_distances.sort();
    println!("Part 2: {}", part2_distances[0]);

    ExitCode::SUCCESS
}
