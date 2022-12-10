use std::{
    env,
    process::ExitCode,
    collections::HashSet,
    io::BufReader,
};
use fs_err::File;
use csv;
use anyhow::{Result, Context};
use itertools::Itertools;

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

fn parse_input_file(filename: &String) -> Result<Vec<(i32, i32, i32, i32)>> {
    let mut output: Vec<(i32, i32, i32, i32)> = Vec::new();

    // Open the file and create the csv reader
    // Have to use `ReaderBuilder` since the regular `Reader` assumes line 1 is a header
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(File::open(filename).context("Error occurred opening File")?));

    // For every row, take each string, split it on the '-' character, and parse each
    // half of that as an i32. Then, push those i32s into a vector
    for row in rdr.records() {
        let mut extension = Vec::new();
        for item in row.context("Error occurred unwrapping string")?.into_iter() {
            for tm in item.split("-").into_iter() {
                extension.push(tm.parse::<i32>().context("Error occurred parsing string to int")?);
            }
        }

        // Take that vector of size 4 and turn it into a 4-tuple
        // and push that 4-tuple into our return vector
        output.push(
            extension
                .into_iter()
                .collect_tuple()
                .context("Error occurred parsing vec to tuple")?,
        );
    }

    Ok(output)
}

// This is basically an implementation of python's <= comparison operator
// for sets
fn set_less_equal(s1: &HashSet<i32>, s2: &HashSet<i32>) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    for elem in s1.into_iter() {
        if !s2.contains(elem) {
            return false;
        }
    }
    true
}

fn vec_from_inclusive_range(start: i32, end: i32) -> Vec<i32> {
    let mut res = Vec::new();
    for val in start..end + 1 {
        res.push(val);
    }
    res
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

    let values = match parse_input_file(&conf.filename) {
        Ok(v) => v,
        Err(_) => return ExitCode::FAILURE,
    };

    let mut part1total = 0;
    let mut part2total = 0;
    for (s1, e1, s2, e2) in values {
        let range1: HashSet<i32> = HashSet::from_iter(vec_from_inclusive_range(s1, e1).into_iter());
        let range2: HashSet<i32> = HashSet::from_iter(vec_from_inclusive_range(s2, e2).into_iter());

        if set_less_equal(&range1, &range2) || set_less_equal(&range2, &range1) {
            part1total += 1;
        }

        let intersection: HashSet<&i32> = range1.intersection(&range2).collect();
        if intersection.len() > 0 {
            part2total += 1;
        }
    }
    println!("Part 1 total: {part1total}");
    println!("Part 2 total: {part2total}");

    ExitCode::SUCCESS
}
