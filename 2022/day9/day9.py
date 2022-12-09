#!/usr/bin/env python3

from __future__ import annotations

from enum import Enum
from math import dist, sqrt
from typing import Set, Tuple, List
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
        self.direction = None

    def reset_position(self) -> None:
        self.x = 0
        self.y = 0
        self.direction = None

    def move(self, dr: Direction) -> None:
        '''Moves the head around, updating the position coordinates appropriately.
        '''
        self.direction = dr
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
        self.max_distance = sqrt(2)

    def _is_same_row(self, h: Head) -> bool:
        return self.x != h.x and self.y == h.y

    def _is_same_col(self, h: Head) -> bool:
        return self.x == h.x and self.y != h.y

    def move(self, h: Head) -> None:
        '''We need to ensure the distance between Head and Tail is never more than
        sqrt(2). If it is, we adjust the location of the Tail.
        '''
        distance = dist((h.x, h.y), (self.x, self.y))
        if distance > self.max_distance:
            if not self._is_same_col(h) and not self._is_same_row(h):
                # if the head goes up or down, we need to match the column
                # if it goes left or right, we need to match the row
                match h.direction:
                    case Direction.UP:
                        self.x = h.x
                        self.y += 1
                    case Direction.DOWN:
                        self.x = h.x
                        self.y -= 1
                    case Direction.LEFT:
                        self.x -= 1
                        self.y = h.y
                    case Direction.RIGHT:
                        self.x += 1
                        self.y = h.y
            elif self._is_same_row(h):
                if h.x - self.x > 0:
                    self.x += 1
                else:
                    self.x -= 1
            elif self._is_same_col(h):
                if h.y - self.y > 0:
                    self.y += 1
                else:
                    self.y -= 1
            self._add_to_visited()

    def reset_position(self) -> None:
        self.x = 0
        self.y = 0
        self._visited.clear()
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
    head = Head()
    tail = Tail()
    print(f'Initial head coordinates: ({head.x}, {head.y})')
    print(f'Initial tail coordinates: ({tail.x}, {tail.y})')
    instructions = parse_input_file(input_file)
    for dr, amt in instructions:
        for _ in range(amt):
            head.move(Direction.letter_to_direction(dr))
            tail.move(head)
    print(f'Final head coordinates: ({head.x}, {head.y})')
    print(f'Final tail coordinates: ({tail.x}, {tail.y})')
    print(f'Part 1: {len(tail.get_visited())}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
