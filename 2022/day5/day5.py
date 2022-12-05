#!/usr/bin/env python3

from typing import Tuple
from sys import argv, exit, stderr
from collections import defaultdict


def parse_instruction_line(instruction: str) -> Tuple[int, str, str]:
    parsed_instruction = instruction.split(";")
    number_of_crates = parsed_instruction[0]
    starting_stack = parsed_instruction[1].split("->")[0]
    destination_stack = parsed_instruction[1].split("->")[1]
    return int(number_of_crates), starting_stack, destination_stack


def parse_state_line(state_input_line: str) -> Tuple[str, str]:
    crate_number = state_input_line.split(":")[0]
    crates = state_input_line.split(":")[1]
    return crate_number, crates


def main(state_file: str, instructions_file: str, part1: bool = True):
    movement_stack = []
    stacks_dict = defaultdict(list)

    # first, parse the state file to populate the stacks dict
    with open(state_file, 'r') as state_input:
        # each line is formatted "#:crate1,crate2,...,crate N"
        for line in state_input:
            crate_number, crates = parse_state_line(line.rstrip())
            for crate in crates.split(","):
                stacks_dict[crate_number].append(crate)

    # next, run through the instruction file manipulating the crates as necessary
    with open(instructions_file, 'r') as instructions:
        # each line is formatted "number of crates;starting stack -> destination stack"
        for instruction in instructions:
            number_of_crates, starting_stack, destination_stack = parse_instruction_line(instruction.rstrip())

            # difference between part 1 and part 2
            # in part 1, the we pop from starting and push to destination immediately
            # in part 2, we pop all crates from starting first, then push them all to destination afterwards
            for _ in range(number_of_crates):
                movement_stack.append(stacks_dict[starting_stack].pop())
                if part1:
                    stacks_dict[destination_stack].append(movement_stack.pop())
            if not part1:
                for _ in range(number_of_crates):
                    stacks_dict[destination_stack].append(movement_stack.pop())

    output = ''
    for stack in stacks_dict.values():
        output += stack[-1]

    print(f'Values on top of stacks: {output}')


if __name__ == '__main__':
    if len(argv) != 3:
        print("ERROR: expected 2 arguments (starting state file, instructions file)", file=stderr)
        exit(1)

    part1 = input("Do you want part1 behavior (Y/N)?: ")
    if part1.upper() == 'Y':
        main(argv[1], argv[2])
    else:
        main(argv[1], argv[2], False)
