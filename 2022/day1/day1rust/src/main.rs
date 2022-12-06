use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
};
use fs_err::File;

// Number of expected command line arguments
const EXPECTED_ARG_NUM: usize = 2;
const FILE_IDX: usize = 1;

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != EXPECTED_ARG_NUM {
            return Err("number of passed in arguments incorrect.");
        }
        Ok(Config { filename: args[FILE_IDX].clone() })
    }
}

fn main() -> ExitCode {
    println!();

    let args: Vec<String> = env::args().collect();
    let conf = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing comand line arguments: {e}");
            return ExitCode::FAILURE;
        }
    };

    process_file(&conf.filename)
}

fn process_file(filename: &String) -> ExitCode {
    let infile = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file `{filename}`: {e}");
            return ExitCode::FAILURE;
        }
    };

    let reader = BufReader::new(infile);
    let mut calories = Vec::new();
    let mut running_sum = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.as_str() == "" {
                    calories.push(running_sum);
                    running_sum = 0;
                } else {
                    match l.parse::<i32>() {
                        Ok(n) => running_sum += n,
                        Err(pe) => {
                            eprintln!("Error parsing `{}` as int: {}", l, pe);
                            return ExitCode::FAILURE;
                        }
                    };
                }
            }
            Err(e) => {
                eprintln!("Error reading line from file: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    if running_sum != 0 {
        calories.push(running_sum);
    }
    calories.sort_by(|a, b| b.cmp(a));
    let top3sum: i32 = calories[..3].iter().sum();
    println!("Most calories: {}", calories[0]); // problem 1
    println!("Top 3 total: {}", top3sum); // problem 2
    ExitCode::SUCCESS
}
