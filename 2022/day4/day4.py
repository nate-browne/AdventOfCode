#!/usr/bin/env python3

import csv
from typing import List, Tuple
from sys import argv, exit, stderr


def parse_input_file(filename: str) -> List[Tuple[int, int, int, int]]:
    output = []
    with open(filename, 'r', newline='') as infile:
        reader = csv.reader(infile, delimiter=',')
        for row in reader:
            extension = []
            for item in row:
                nums = item.split('-')
                extension.extend([int(num) for num in nums])
            output.append(tuple(extension))
    return output


def main(filename: str):
    values = parse_input_file(filename)

    part1total = 0
    part2total = 0
    # Idea here: we make sets of the ranges and look for
    # the overlap. This means our ranges have to be inclusive
    # s = start, e = end
    for s1, e1, s2, e2 in values:
        range1 = set(range(s1, e1 + 1))
        range2 = set(range(s2, e2 + 1))

        # how many overlaps are complete?
        if range1 <= range2 or range2 <= range1:
            part1total += 1
        # how many ranges overlap at all?
        if range1 & range2:
            part2total += 1
    print(f'Part 1 total: {part1total}')
    print(f'Part 2 total: {part2total}')


if __name__ == '__main__':
    if len(argv) != 2:
        print("ERROR: Expected 1 argument (filename)", file=stderr)
        exit(1)
    main(argv[1])
