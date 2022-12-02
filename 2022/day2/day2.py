#!/usr/bin/env python3

from sys import argv, stderr, exit

from classes import PlayerMove, OpponentMove

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
