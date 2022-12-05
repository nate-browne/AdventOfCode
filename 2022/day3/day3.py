#!/usr/bin/env python3
from typing import List
from string import ascii_letters
from sys import argv, stderr, exit


class Day3Mapper(object):

    def __init__(self):
        self.mappings = {lttr: ind + 1 for (ind, lttr) in enumerate(ascii_letters)}
        self.sum = 0

    def process_line(self, line: str):
        str_length = len(line)
        bag1 = line[:str_length // 2]
        bag2 = line[str_length // 2:].rstrip()
        overlap = set(bag1) & set(bag2)
        for item in overlap:
            self.sum += self.mappings[item]

    def process_three_lines(self, lines: List[str]):
        overlap = set(lines[0]) & set(lines[1]) & set(lines[2])
        for item in overlap:
            self.sum += self.mappings[item]


def main(input_file: str):
    with open(input_file, 'r') as infile:
        mapper = Day3Mapper()
        # part 1
        for line in infile:
            mapper.process_line(line)
        print(f'Sum is (part 1): {mapper.sum}')

        # part 2
        infile.seek(0)
        mapper.sum = 0
        lines = []
        for ind, line in enumerate(infile):
            if ind % 3 == 0 and ind != 0:
                mapper.process_three_lines(lines)
                lines.clear()

            lines.append(line.rstrip())
        mapper.process_three_lines(lines)
        print(f'Sum is (part 2): {mapper.sum}')


if __name__ == "__main__":
    if len(argv) != 2:
        print("ERROR: Expected 1 argument (filename)", file=stderr)
        exit(1)
    main(argv[1])
