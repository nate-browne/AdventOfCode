#!/usr/bin/env python3

from __future__ import annotations

from sys import argv, exit, stderr

expected_args = 2


class Monkey(object):
    def __init__(self, monkey_dict: dict) -> Monkey:
        pass


def main(input_file: str):
    pass


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
