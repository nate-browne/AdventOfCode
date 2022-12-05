use std::{env, io::{stdout, Write, stdin, BufRead, BufReader}};
use std::process::{ExitCode, exit};
use fs_err::File;
use std::collections::BTreeMap;
use anyhow::{Result, Context};

// Number of expected command line arguments
const EXPECTED_ARG_NUM: usize = 3;
const STATE_FILE_IDX: usize = 1;
const INSTR_FILE_IDX: usize = 2;

struct Config {
    state_filename: String,
    instr_filename: String, 
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != EXPECTED_ARG_NUM {
            return Err("number of passed in arguments incorrect.");
        }
        Ok(Config {
            state_filename: args[STATE_FILE_IDX].clone(),
            instr_filename: args[INSTR_FILE_IDX].clone()
        })
    }
}

fn grab_input(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut option = String::new();

    match stdin().read_line(&mut option) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error occurred while reading string: {e}");
            exit(1);
        }
    };
    String::from(option.trim())
}

fn parse_state_line(state_input_line: String) -> (String, String) {
    let split_ln: Vec<&str> = state_input_line.split(":").collect();
    (split_ln[0].to_string(), split_ln[1].to_string())
}

fn parse_instruction_line(instruction: String) -> Result<(i32, String, String)> {
    let parsed_instruction: Vec<&str> = instruction.split(";").collect();
    let number_of_crates = parsed_instruction[0].parse::<i32>()?;

    let stack_info: Vec<&str> = parsed_instruction[1].split("->").collect();
    let starting_stack = stack_info[0].to_string();
    let destination_stack = stack_info[1].to_string();

    Ok((number_of_crates, starting_stack, destination_stack))
}

fn fill_stacks_map(state_filename: &String) -> Result<BTreeMap<String, Vec<String>>> {
    let mut stacks_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let reader = BufReader::new(
        File::open(state_filename).context("Error occurred opening starting state file")?
    );

    for line in reader.lines() {
        let (crate_number, crates) = parse_state_line(line.unwrap());

        if !stacks_map.contains_key(&crate_number) {
            stacks_map.insert(crate_number.clone(), Vec::new());
        }

        let st: &mut Vec<String> = match stacks_map.get_mut(&crate_number) {
            Some(s) => s,
            None => panic!("This shouldn't be possible; no matching stack for key given"),
        };

        for cr in crates.split(",").into_iter() {
            st.push(cr.to_string());
        }
    }
    Ok(stacks_map)
}

fn run_simulation(stacks_map: &mut BTreeMap<String, Vec<String>>, instructions_file: &String, part1behavior: bool) -> Result<()> {
    let mut movement_stack = vec![];

    let reader = BufReader::new(
        File::open(instructions_file).context("Error occurred opening instructions file")?
    );

    for line in reader.lines() {
        let (number_of_crates, starting_stack, destination_stack) = parse_instruction_line(line.unwrap())?;

        for _ in 0..number_of_crates {
            let mut st = match stacks_map.get_mut(&starting_stack) {
                Some(s) => s,
                None => panic!("ruh roh this shouldn't have happened"),
            };
            let mut val = match st.pop() {
                Some(v) => v,
                None => panic!("ruh roh this shouldn't have happened"),
            };
            movement_stack.push(val);

            if part1behavior {
                st = match stacks_map.get_mut(&destination_stack) {
                    Some(s) => s,
                    None => panic!("ruh roh this shouldn't have happened"),
                };

                val = match movement_stack.pop() {
                    Some(v) => v,
                    None => panic!("ruh roh this shouldn't have happened"),
                };
                st.push(val);
            }
        }

        if !part1behavior {
            for _ in 0..number_of_crates {
                let st = match stacks_map.get_mut(&destination_stack) {
                    Some(s) => s,
                    None => panic!("ruh roh this shouldn't have happened"),
                };

                let val = match movement_stack.pop() {
                    Some(v) => v,
                    None => panic!("ruh roh this shouldn't have happened"),
                };
                st.push(val);
            }
        }
    }
    let mut output = String::new();

    for (_, val) in stacks_map.into_iter() {
        let add = match val.last() {
            Some(v) => v,
            None => panic!("no item at top of this stack!"),
        };
        output.push_str(add.as_str());
    }
    println!("Values on tops of stacks: {output}");

    Ok(())
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

    let part1 = grab_input("Do you want part1 behavior (Y/N)?: ");
    let mut part1behavior = false;
    match part1.to_uppercase().as_str() {
        "Y" => part1behavior = true,
        _ => ()
    }

    let mut stacks_map = match fill_stacks_map(&conf.state_filename) {
        Ok(sm) => sm,
        Err(e) => {
            eprintln!("Error occurred parsing starting state file: {e}");
            return ExitCode::FAILURE;
        }
    };

    match run_simulation(&mut stacks_map, &conf.instr_filename, part1behavior) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error occurred running instructions: {e}");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
