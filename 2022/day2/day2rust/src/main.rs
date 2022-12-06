use std::{
    env,
    io::{BufRead, BufReader},
    process::ExitCode,
    collections::{HashMap, HashSet},
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

struct OpponentMove {
    translated_move: String,
}

impl OpponentMove {
    fn new(move_letter: &str) -> OpponentMove {
        let move_mapping = HashMap::from([
            ("A", "rock"),
            ("B", "paper"),
            ("C", "scissors"),
        ]);
        OpponentMove { translated_move: move_mapping[move_letter].to_string() }
    }
}

struct PlayerMove {
    translated_move: String,
    move_score: i32,
}

impl PlayerMove {
    fn determine_player_move(desired_result: &str, om: &OpponentMove) -> String {
        let losing_combo = HashMap::from([
            ("rock", "scissors"),
            ("scissors", "paper"),
            ("paper", "rock"),
        ]);
        let winning_combo = HashMap::from([
            ("rock", "paper"),
            ("paper", "scissors"),
            ("scissors", "rock"),

        ]);
        if desired_result == "lose" {
            return losing_combo[&om.translated_move.as_str()].to_string();
        } else if desired_result == "win" {
            return winning_combo[&om.translated_move.as_str()].to_string();
        } else {
            return om.translated_move.clone();
        }
    }

    fn new(move_letter: &str, om: &OpponentMove) -> PlayerMove {
        let result_mapping = HashMap::from([
            ("X", "lose"),
            ("Y", "draw"),
            ("Z", "win"),
        ]);
        let move_score = HashMap::from([
            ("rock", 1),
            ("paper", 2),
            ("scissors", 3),
        ]);
        let translated_move = PlayerMove::determine_player_move(result_mapping[move_letter], om);
        PlayerMove {
            translated_move: translated_move.clone(),
            move_score: move_score[translated_move.as_str()],
        }
    }

    fn score_round(&self, om: &OpponentMove) -> i32 {
        let player_wins = HashSet::from(["paperrock", "rockscissors", "scissorspaper"]);
        let player_draws = HashSet::from(["paperpaper", "rockrock", "scissorsscissors"]);

        let combined_move = self.translated_move.clone() + &om.translated_move;

        if player_wins.contains(combined_move.as_str()) {
            return self.move_score + 6;
        } else if player_draws.contains(combined_move.as_str()) {
            return self.move_score + 3;
        } else {
            return self.move_score;
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

    let reader = BufReader::new(infile);
    let mut scores: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let moves: Vec<&str> = l.split_whitespace().collect();
                let om = OpponentMove::new(moves[0]);
                let pm = PlayerMove::new(moves[1], &om);
                scores.push(pm.score_round(&om));
            }
            Err(e) => {
                eprintln!("Error parsing line: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    let score: i32 = scores.iter().sum();
    println!("Your score is: {}", score);
    ExitCode::SUCCESS
}
