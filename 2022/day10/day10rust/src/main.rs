use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
    collections::{HashMap, HashSet},
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

fn process_file(input_file: &String) -> Result<Vec<(String, String)>> {
    let mut result = vec![];

    let infile = File::open(input_file)
        .context("Error occurred opening file")?;
    let reader = BufReader::new(infile);

    for line in reader.lines() {
        let mut ln = line?;
        if ln.as_str() == "noop" {
            ln += " 0";
        }
        let v: Vec<&str> = ln.split(" ").collect();
        result.push((v[0].to_string(), v[1].to_string()));
    }

    Ok(result)
}

fn part1(instructions: &Vec<(String, String)>, command_map: &HashMap<String, i32>) -> Result<i32> {
    let mut signal_strength: i32 = 0;
    let mut x_register = 1;
    let mut tick: i32 = 0;
    let ticks: HashSet<i32> = HashSet::from([20, 60, 100, 140, 180, 220]);

    for (instr, arg) in instructions {
        let num_ticks = command_map[instr];
        for val in 0..num_ticks {
            tick += 1;
            if ticks.contains(&tick) {
                signal_strength += x_register * tick;
            }
            if instr.as_str() == "addx" && val == num_ticks - 1 {
                x_register += arg.parse::<i32>()?;
            }
        }
    }
    Ok(signal_strength)
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

    let instructions = match process_file(&conf.input_file) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Error occured processing input file: {e}");
            return ExitCode::FAILURE;
        }
    };

    let command_map: HashMap<String, i32> = HashMap::from([
        ("addx".to_string(), 2),
        ("noop".to_string(), 1),
    ]);

    let part1_answer = match part1(&instructions, &command_map) {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("Error solving part 1: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Part 1 answer: {part1_answer}");

    ExitCode::SUCCESS
}
