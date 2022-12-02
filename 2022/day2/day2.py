#!/usr/bin/env python3

from sys import argv, stderr, exit

from classes import PlayerMove, OpponentMove

def determine_player_move(desired_result: str, om: OpponentMove) -> str:
    losing_combo = {
        'rock': 'scissors',
        'scissors': 'paper',
        'paper': 'rock',
    }
    if desired_result == 'lose':
        return losing_combo[om.translated_move]
    elif desired_result == 'draw': # simple case, we need to mirror the opponent
        return om.translated_move
    else:
        pass

def process_line(line: str) -> int:
    moves = line.split()
    om = OpponentMove(moves[0])
    pm = PlayerMove(moves[1], om)
    return pm.score_round(om)

def main(input_file: str):
    with open(input_file, 'r') as infile:
        round_scores = [process_line(line) for line in infile]
    print(f'Your score is: {sum(round_scores)}')

if __name__ == "__main__":
    if len(argv) != 2:
        print("ERROR: Expected 1 argument (filename)", file=stderr)
        exit(1)
    main(argv[1])
