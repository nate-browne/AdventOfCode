use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
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

fn build_tree_grid(input_file: &String) -> Result<Vec<Vec<i32>>> {
    let infile = File::open(input_file)
        .context("Error occurred opening input file.")?;
    let mut res = Vec::new();

    let reader = BufReader::new(infile);

    for line in reader.lines() {
        let ln = line?;
        let mut tmp = Vec::new();
        for ch in ln.chars() {
            tmp.push(ch as i32 - '0' as i32);
        }
        res.push(tmp);
    }
    Ok(res)
}

fn count_visible_trees(tree_grid: &Vec<Vec<i32>>) -> i32 {
    let mut visible: i32 = 0;

    for row in 0..tree_grid[0].len() {
        for col in 0..tree_grid.len() {
            let current_tree = tree_grid[row][col];

            // edges are visible by default
            // an edge tree has coordinates (0, y) or (x, 0)
            if row == 0 || col == 0 || row == tree_grid[0].len() - 1 || col == tree_grid.len() - 1 {
                visible += 1;
                continue;
            }
            let mut is_visible: bool = false;

            // north
            for val in (0..row).rev() {
                if current_tree <= tree_grid[val][col] {
                    break;
                }
                if val == 0 {
                    is_visible = true;
                }
            }

            // east
            for val in col + 1..tree_grid.len() {
                if current_tree <= tree_grid[row][val] {
                    break;
                }
                if val == tree_grid.len() - 1 {
                    is_visible = true;
                }
            }
            // south
            for val in row + 1..tree_grid[0].len() {
                if current_tree <= tree_grid[val][col] {
                    break;
                }
                if val == tree_grid[0].len() - 1 {
                    is_visible = true;
                }
            }
            // west
            for val in (0..col).rev() {
                if current_tree <= tree_grid[row][val] {
                    break;
                }
                if val == 0 {
                    is_visible = true;
                }
            }

            if is_visible {
                visible += 1;
            }
        }
    }
    visible
}

fn evaluate_tree_scores(tree_grid: &Vec<Vec<i32>>) -> i32 {
    let mut highest_score = 0;
    highest_score
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

    let tree_grid = match build_tree_grid(&conf.input_file) {
        Ok(tg) => tg,
        Err(e) => {
            eprintln!("Error building tree grid: {e}");
            return ExitCode::FAILURE;
        }
    };

    let part1 = count_visible_trees(&tree_grid);
    println!("part 1: {part1}");

    ExitCode::SUCCESS
}
