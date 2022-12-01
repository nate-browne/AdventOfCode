#!/usr/bin/env python3

from sys import argv, stderr, exit

def main(input_file: str):
    with open(input_file, 'r') as infile:
        calories = []
        running_sum = 0
        for line in infile:
            if line == '\n':
                calories.append(running_sum)
                running_sum = 0
            else:
                running_sum += int(line)
        # if we reach EOF and there's still a sum, that's the last elf
        if running_sum != 0:
            calories.append(running_sum)
        calories.sort(reverse=True)
        print(f'Most calories: {calories[0]}') # part 1 answer
        print(f'Top 3 total: {sum(calories[:3])}') # part 2

if __name__ == "__main__":
    if len(argv) != 2:
        print("ERROR: Expected 1 argument (filename)", file=stderr)
        exit(1)
    main(argv[1])