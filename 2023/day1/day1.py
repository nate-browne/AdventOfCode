#!/usr/bin/env python3

from typing import Tuple, Optional
from sys import argv, stderr, exit

FORWARDS_MAP = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

BACKWARDS_MAP = {
    "eno": "1",
    "owt": "2",
    "eerht": "3",
    "ruof": "4",
    "evif": "5",
    "xis": "6",
    "neves": "7",
    "thgie": "8",
    "enin": "9",
}

def _find_first_digit(line: str, backwards: bool) -> Tuple[int, str]:
    str_ind, str_char = _find_first_string_digit(line, backwards)
    num_ind, num_char = _find_first_numerical_digit(line)
    if num_ind is not None and str_ind is not None:
        return (num_ind, num_char) if num_ind < str_ind else (str_ind, str_char)
    elif num_ind is None:
        return (str_ind, str_char)
    elif str_ind is None:
        return (num_ind, num_char)
    else:
        return (None, None)

def _find_first_string_digit(line: str, backwards: bool) -> Optional[Tuple[int, str]]:
    index1, index2 = 0, 5
    if index2 > len(line):
        index2 = len(line)

    while index2 <= len(line):
        if not backwards:
            for key in FORWARDS_MAP.keys():
                if line[index1:index2].startswith(key):
                    return index1, FORWARDS_MAP[key]
        else:
            for key in BACKWARDS_MAP.keys():
                if line[index1:index2].startswith(key):
                    return index1, BACKWARDS_MAP[key]
        index1 += 1
        index2 += 1
    return None, None

def _find_first_numerical_digit(line: str) -> Optional[Tuple[int, str]]:
    for ind, char in enumerate(line):
        try:
            int(char)
            return ind, char
        except ValueError:
            continue
    return None, None

def find_two_digit_number(line: str) -> int:
    result = ''
    first_ind, first_char = _find_first_digit(line, False)
    last_ind, last_char = _find_first_digit(line[::-1], True)
    result += first_char
    return int(result)

if __name__ == "__main__":
    if len(argv) != 2:
        print("ERROR: expected 1 argument (filename)", file=stderr)
        exit(1)

    with open(argv[1], 'r') as infile:
        values = infile.read().splitlines()
        with open('outfile.txt', 'w') as outfile:
            outfile.write('\n'.join(map(lambda x: str(find_two_digit_number(x)), values)))
        print(sum(map(lambda x: find_two_digit_number(x), values)))
