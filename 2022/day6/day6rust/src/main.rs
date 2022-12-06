use std::{env, io::{BufRead, BufReader}};
use std::process::ExitCode;
use fs_err::File;
use std::collections::HashSet;
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

fn solver(input_file: &String, pointer2start: usize) -> Result<usize> {
    let mut pointer1 = 0;
    let mut pointer2 = pointer2start;
    let infile = File::open(input_file)
        .context("Error occurred opening input file")?;
    let reader = BufReader::new(infile);

    // This for loop only runs once since each file only has one line in it
    for line in reader.lines() {
        let ln = line.unwrap();
        while pointer2 < ln.len() {
            let pointed_chars = &ln[pointer1..pointer2];
            let charset: HashSet<char> = pointed_chars.chars().collect();

            if pointed_chars.len() == charset.len() {
                break;
            }
            pointer1 += 1;
            pointer2 += 1;
        }
    }
    Ok(pointer2)
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

    let part1_answer = match solver(&conf.input_file, 4) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error occurred solving part 1: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Part 1 answer: {part1_answer}");

    let part2_answer = match solver(&conf.input_file, 14) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error occurred solving part 2: {e}");
            return ExitCode::FAILURE
        }
    };

    println!("Part 2 answer: {part2_answer}");

    ExitCode::SUCCESS
}
