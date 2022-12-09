#!/usr/bin/env python3

from __future__ import annotations

from enum import Enum
from typing import Set, Tuple, List, Union
from sys import argv, exit, stderr

expected_args = 2


class Direction(Enum):
    UP = 0
    DOWN = 1
    LEFT = 2
    RIGHT = 3

    @classmethod
    def letter_to_direction(cls, lt: str) -> Direction:
        match lt.upper():
            case 'U':
                return cls.UP
            case 'D':
                return cls.DOWN
            case 'L':
                return cls.LEFT
            case 'R':
                return cls.RIGHT


class Head(object):
    def __init__(self) -> Head:
        self.x = 0
        self.y = 0

    def move(self, dr: Direction) -> None:
        '''Moves the head around, updating the position coordinates appropriately.
        '''
        match dr:
            case Direction.UP:
                self.y += 1
            case Direction.DOWN:
                self.y -= 1
            case Direction.LEFT:
                self.x -= 1
            case Direction.RIGHT:
                self.x += 1


class Tail(object):
    def __init__(self) -> Tail:
        self._visited: Set[Tuple[int, int]] = set()
        self.x = 0
        self.y = 0
        self._add_to_visited()

    def move(self, other: Union[Head, Tail]) -> None:
        '''We need to ensure the distance between Head and Tail is never more than
        sqrt(2). If it is, we adjust the location of the Tail.
        '''
        x_dist = other.x - self.x
        y_dist = other.y - self.y
        if abs(x_dist) >= 2 or abs(y_dist) >= 2:
            if x_dist > 0:
                self.x += 1
            elif x_dist < 0:
                self.x -= 1
            if y_dist > 0:
                self.y += 1
            elif y_dist < 0:
                self.y -= 1
        self._add_to_visited()

    def get_visited(self) -> Set[Tuple[int, int]]:
        return self._visited

    def _add_to_visited(self) -> None:
        self._visited.add((self.x, self.y))


def parse_input_file(input_file: str) -> List[Tuple[str, int]]:
    result = []
    with open(input_file, 'r') as infile:
        for line in infile:
            line_items = line.strip().split()
            result.append((line_items[0], int(line_items[1])))
    return result


def main(input_file: str):
    instructions = parse_input_file(input_file)
    head = Head()
    tail = Tail()
    for dr, amt in instructions:
        for _ in range(amt):
            head.move(Direction.letter_to_direction(dr))
            tail.move(head)
    print(f'Part 1: {len(tail.get_visited())}')

    rope_snake: List[Union[Head, Tail]] = []
    rope_snake.append(Head())
    for _ in range(9):
        rope_snake.append(Tail())

    for dr, amt in instructions:
        for _ in range(amt):
            for ind, item in enumerate(rope_snake):
                if isinstance(item, Head):
                    item.move(Direction.letter_to_direction(dr))
                elif isinstance(item, Tail):
                    item.move(rope_snake[ind - 1])

    print(f'Part 2: {len(rope_snake[-1].get_visited())}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
