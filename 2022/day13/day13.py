#!/usr/bin/env python3

from math import prod
from json import loads
from typing import Union
from functools import cmp_to_key
from sys import argv, exit, stderr

expected_args = 2


def compare_lines(left: Union[int, list], right: Union[int, list]) -> int:
    '''C-style comparator function comparing entries.\n
    Ultimately tries to compare two integers by taking their difference, and recursively breaks
    down the input until two integers are reached.
    '''
    if isinstance(left, int) and isinstance(right, int):
        return left - right

    left = left if isinstance(left, list) else [left]
    right = right if isinstance(right, list) else [right]

    for lf, rh in zip(left, right):
        tmp_res = compare_lines(lf, rh)
        if tmp_res != 0:
            return tmp_res
    return len(left) - len(right)  # ensure that left is shorter


def part1(lines) -> int:
    ans = []
    for ind, (left, right) in enumerate(lines):
        if compare_lines(left, right) <= 0:
            ans.append(ind + 1)
    return sum(ans)


def part2(lines) -> int:
    dividers = [[[2]], [[6]]]
    packets = [packet for group in lines for packet in group]
    packets.extend(dividers)
    packets.sort(key=cmp_to_key(compare_lines))

    ans = []
    for ind, packet in enumerate(packets):
        if packet in dividers:
            ans.append(ind + 1)
    return prod(ans)


def main(input_file: str):
    with open(input_file, 'r') as infile:
        tmp = infile.read().split('\n\n')
        if not tmp[-1]:
            tmp = tmp[:-1]
        tmp = [group.splitlines() for group in tmp]

    lines = [[loads(s) for s in group] for group in tmp]

    print(f'Part 1: {part1(lines)}')
    print(f'Part 2: {part2(lines)}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
