#!/usr/bin/env python3

from typing import List, Tuple
from string import ascii_lowercase
from sys import argv, exit, stderr
from collections import namedtuple, deque

expected_args = 2

Point = namedtuple('Point', ['x', 'y'])
Grid = List[List[str]]


def create_grid(input_file: str) -> Tuple[Grid, Point, Point]:
    result: List[List[str]] = []
    with open(input_file, 'r') as infile:
        for line in infile:
            result.append(list(line.strip()))

    start, end = None, None
    for x in range(len(result)):
        for y in range(len(result[0])):
            if result[x][y] == 'S':
                start = Point(x, y)
            if result[x][y] == 'E':
                end = Point(x, y)
    return result, start, end


def point_in_bounds(point: Point, grid: Grid) -> bool:
    return 0 <= point.x < len(grid) and 0 <= point.y < len(grid[0])


def BFS(grid: Grid, start: Point, end: Point) -> int:
    seen = set()
    queue = deque()
    queue.append((start, 0))

    while len(queue):
        current, dist = queue.popleft()

        if current in seen:
            continue

        seen.add(current)
        current_height = grid[current.x][current.y]

        if current == end:
            return dist

        # check neighbors
        for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nxt = Point(dx + current.x, dy + current.y)
            if point_in_bounds(nxt, grid):
                new_height = grid[nxt.x][nxt.y]
                if letter_to_height(new_height) - letter_to_height(current_height) <= 1:
                    queue.append((nxt, dist + 1))
    return 1_000_000_000  # no path exists


def letter_to_height(ltr: str) -> int:
    letter_to_height_map = {letter: num for (letter, num) in zip(ascii_lowercase, range(26))}
    letter_to_height_map['S'] = 0
    letter_to_height_map['E'] = 25
    return letter_to_height_map[ltr]


def main(input_file: str):
    grid, start, end = create_grid(input_file)

    print(f'Part 1: {BFS(grid, start, end)}')

    # Construct a list of all possible starting points (points with height 0)
    starting_points = [Point(x, y) for x in range(len(grid)) for y in range(len(grid[0])) if letter_to_height(grid[x][y]) == 0]

    # for each point, run BFS to get distance to end. Then, sort and pick the shortest
    part2_distances = [BFS(grid, start_point, end) for start_point in starting_points]
    part2_distances.sort()
    part2 = part2_distances[0]
    print(f'Part 2: {part2}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
