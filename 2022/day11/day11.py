#!/usr/bin/env python3

from __future__ import annotations

import json
from math import prod
from typing import Dict
from collections import deque
from sys import argv, exit, stderr

expected_args = 2


class Monkey(object):

    @staticmethod
    def parse_operation(op_str: str):
        '''string is in format new=old<op>#\n
        where <op> is in (%,*,+,-,/) and # is a numerical value.\n
        This function returns a function
        '''
        # nasty little check for old * old
        if op_str.split('=')[1] == 'old*old':
            return lambda x: x * x

        op_and_num = op_str.split('=')[1].split('old')[1]

        match op_and_num[0]:
            case '+':
                return lambda x: x + int(op_and_num[1:])
            case '-':
                return lambda x: x - int(op_and_num[1:])
            case '*':
                return lambda x: x * int(op_and_num[1:])
            case '/':
                return lambda x: x // int(op_and_num[1:])
            case '%':
                return lambda x: x % int(op_and_num[1:])

    def __init__(self, monkey_dict: dict) -> Monkey:
        self.items = deque(monkey_dict['starting_items'])
        self.operation = Monkey.parse_operation(monkey_dict['operation'])
        # needed in part 2. this is super gross but honestly idgaf
        self.test_val = int(monkey_dict['test']['operation'].split('=')[1].split('%')[1])
        self.test = {
            'operation': Monkey.parse_operation(monkey_dict['test']['operation']),
            True: int(monkey_dict['test']['true']),
            False: int(monkey_dict['test']['false']),
        }
        self.num_inspections = 0

    def __repr__(self) -> str:
        return f'{self.items}'

    def inspect(self, other_monkeys: Dict[Monkey], part1: bool, mod_val: int) -> None:
        '''Function that goes through the held items and performs the inspection
        '''
        while len(self.items):
            self.num_inspections += 1
            current_item = self.items.popleft()
            current_item = self.operation(current_item)
            if part1:
                current_item //= 3
            else:
                current_item %= mod_val
            check = self.test['operation'](current_item) == 0
            other_monkeys[self.test[check]].receive_item(current_item)

    def receive_item(self, item: int) -> None:
        self.items.append(item)


def parse_input_file(input_file: str) -> Dict[Monkey]:
    monkeys = {}
    with open(input_file, 'r') as infile:
        monkeys_dict = json.load(infile)
        for ind, monkey in enumerate(monkeys_dict.values()):
            monkeys[ind] = Monkey(monkey)
    return monkeys


def run_simulation(monkeys: Dict[Monkey], num_times: int, part1: bool) -> None:
    mod_val = prod(monkey.test_val for monkey in monkeys.values())
    for n in range(num_times):
        for monkey in monkeys.values():
            monkey.inspect(monkeys, part1, mod_val)
        if n % 1_000 == 0 and n != 0:
            print('.')


def main(input_file: str):
    if '.json' not in input_file:
        print('ERROR: expected input file to be a json file', file=stderr)
        exit(1)

    monkeys = parse_input_file(input_file)
    run_simulation(monkeys, 20, True)

    num_inspections = [m.num_inspections for m in monkeys.values()]
    num_inspections.sort(reverse=True)
    print(f'Part 1: {prod(num_inspections[:2])}')

    monkeys = parse_input_file(input_file)
    run_simulation(monkeys, 10_000, False)

    num_inspections = [m.num_inspections for m in monkeys.values()]
    num_inspections.sort(reverse=True)
    print(f'Part 2: {prod(num_inspections[:2])}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
