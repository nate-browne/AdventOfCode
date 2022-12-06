use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
    collections::{HashMap, HashSet},
    convert::TryInto,
};
use fs_err::File;
use intersection::hash_set;

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

struct Day3Mapper {
    sum: i32,
    mappings: HashMap<String, i32>,
}

impl Day3Mapper {
    fn new() -> Day3Mapper {
        // This is my way of doing what python makes really simple.
        // For the record, the equivalent python constructor can be found over in `day3.py`
        let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        let mut alpha_map = Vec::new();
        for (ind, ch) in alpha.enumerate() {
            alpha_map.push((ch.to_string(), ind as i32 + 1));
        }

        let len = alpha_map.len();

        let arr_map: [(String, i32); 52] = match alpha_map.try_into() {
            Ok(am) => am,
            Err(_) => panic!("Expected vector of length 52, received length {}", len),
        };

        Day3Mapper {
            sum: 0,
            mappings: HashMap::from(arr_map),
        }
    }

    fn process_line(&mut self, line: &String) {
        let line_len = line.len();
        let bag1 = &line[..line_len / 2];
        let set1: HashSet<char> = HashSet::from_iter(bag1.chars());
        let bag2 = &line[line_len / 2..];
        let set2: HashSet<char> = HashSet::from_iter(bag2.chars());
        let overlap = set1.intersection(&set2).collect::<Vec<&char>>();

        match self.mappings.get(&overlap[0].to_string()) {
            Some(val) => self.sum += val,
            None => panic!("no matching value found, strange."),
        };
    }

    fn process_three_lines(&mut self, lines: &Vec<String>) {
        let set1: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let set2: HashSet<char> = HashSet::from_iter(lines[1].chars());
        let set3: HashSet<char> = HashSet::from_iter(lines[2].chars());
        let sets = [set1, set2, set3];

        let overlap = hash_set::intersection(sets).into_iter().collect::<Vec<char>>();

        match self.mappings.get(&overlap[0].to_string()) {
            Some(val) => self.sum += val,
            None => panic!("no matching value found, strange."),
        }
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

    let mut mapper = Day3Mapper::new();

    let reader = BufReader::new(infile);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                mapper.process_line(&l);
                lines.push(l.clone());
            }
            Err(e) => {
                eprintln!("Error parsing line: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    println!("Sum is (part 1): {}", mapper.sum);

    mapper.sum = 0;
    let mut three_lines: Vec<String> = Vec::new();
    for (ind, line) in lines.iter().enumerate() {
        if ind % 3 == 0 && ind != 0 {
            mapper.process_three_lines(&three_lines);
            three_lines.clear();
        }
        three_lines.push(line.clone());
    }
    mapper.process_three_lines(&three_lines);
    println!("Sum is (part 2): {}", mapper.sum);

    ExitCode::SUCCESS
}
