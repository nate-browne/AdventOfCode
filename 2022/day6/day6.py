#!/usr/bin/env python3

from sys import argv, exit, stderr

expected_args = 2


def main(input_file: str):
    with open(input_file, 'r') as infile:
        pointer1, pointer2 = 0, 4
        buffer = infile.read()

        while pointer2 < len(buffer):
            pointed_chars = buffer[pointer1:pointer2]
            if len(set(pointed_chars)) == len(pointed_chars):  # if this is true, all chars in sequence are unique
                break
            pointer1 += 1
            pointer2 += 1
        print(f'Part 1 answer: {pointer2}')


if __name__ == '__main__':
    if len(argv) != expected_args:
        print(f"ERROR: expected {expected_args - 1} arguments ()", file=stderr)
        exit(1)
    main(argv[1])
