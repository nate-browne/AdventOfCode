#!/usr/bin/env python3

from typing import List, Tuple
from sys import argv, exit, stderr

expected_args = 2

command_map = {
    'addx': 2,
    'noop': 1,
}


def process_file(input_file: str) -> List[Tuple[str, str]]:
    result = []
    with open(input_file, 'r') as infile:
        for line in infile:
            line = line.strip()
            if line == 'noop':
                line += ' 0'
            item = tuple(line.split())
            result.append(item)
    return result


def part1(instructions: List[Tuple[str, str]]) -> int:
    signal_strength = 0
    x_register = 1
    tick = 0
    ticks = {20, 60, 100, 140, 180, 220}
    for instr, arg in instructions:
        num_ticks = command_map[instr]
        for val in range(num_ticks):
            tick += 1
            if tick in ticks:
                signal_strength += x_register * tick
            if instr == 'addx' and val == num_ticks - 1:
                x_register += int(arg)
    return signal_strength


def part2(instructions: List[Tuple[str, str]]):
    tick = 0
    x_register = 1
    output = []
    for instr, arg in instructions:
        num_ticks = command_map[instr]
        for _ in range(num_ticks):
            pixel_to_draw = tick % 40
            if (x_register - 1) <= pixel_to_draw <= (x_register + 1):
                output.append("#")
            else:
                output.append(".")
            if len(output) >= 40:
                print(''.join(output))
                output.clear()
            tick += 1



def main(input_file: str):
    instructions = process_file(input_file)
    print(f'Part 1: {part1(instructions)}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
