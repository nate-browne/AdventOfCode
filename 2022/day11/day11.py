#!/usr/bin/env python3

from __future__ import annotations

import json
from typing import Dict
from collections import deque
from sys import argv, exit, stderr

expected_args = 2


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


class Monkey(object):
    def __init__(self, monkey_dict: dict) -> Monkey:
        self.items = deque(monkey_dict['starting_items'])
        self.operation = parse_operation(monkey_dict['operation'])
        self.test = {
            'operation': parse_operation(monkey_dict['test']['operation']),
            True: int(monkey_dict['test']['true']),
            False: int(monkey_dict['test']['false']),
        }
        self.num_inspections = 0

    def __repr__(self) -> str:
        return f'{self.items}'

    def inspect(self, other_monkeys: Dict[Monkey]) -> None:
        '''Function that goes through the held items and performs the inspection
        '''
        while len(self.items):
            self.num_inspections += 1
            current_item = self.items.popleft()
            current_item = self.operation(current_item)
            current_item //= 3
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


def run_simulation(monkeys: Dict[Monkey], num_times: int) -> None:
    for _ in range(num_times):
        for monkey in monkeys.values():
            monkey.inspect(monkeys)


def main(input_file: str):
    if '.json' not in input_file:
        print('ERROR: expected input file to be a json file', file=stderr)
        exit(1)

    monkeys = parse_input_file(input_file)
    run_simulation(monkeys, 20)

    num_inspections = [m.num_inspections for m in monkeys.values()]
    num_inspections.sort(reverse=True)
    print(f'Part 1: {num_inspections[0] * num_inspections[1]}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
