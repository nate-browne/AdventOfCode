#!/usr/bin/env python3

from typing import List
from sys import argv, exit, stderr

expected_args = 2


def count_visible_trees(tree_grid: List[List[int]]) -> int:
    '''Part 1. Count how many trees are visible from the outside.\n
    A tree is visible from a direction if there are no trees at the same height or taller
    when looked at from a direction.
    '''
    visible = 0
    for row in range(len(tree_grid[0])):  # x direction, moving along the length of a particular inner array
        for col in range(len(tree_grid)):  # y direction, moving along the inner arrays themselves
            current_tree = tree_grid[row][col]
            # edges are visible by default
            # an edge tree has coordinates (0, y) or (x, 0)
            if row == 0 or col == 0 or row == len(tree_grid[0]) - 1 or col == len(tree_grid) - 1:
                visible += 1
                continue

            is_visible = False
            # north
            for val in range(row - 1, -1, -1):
                if current_tree <= tree_grid[val][col]:
                    break
                if val == 0:  # we got to the edge, so the tree is visible from this direction
                    is_visible = True
            # east
            for val in range(col + 1, len(tree_grid)):
                if current_tree <= tree_grid[row][val]:
                    break
                if val == len(tree_grid) - 1:  # we got to the edge, so the tree is visible from this direction
                    is_visible = True
            # south
            for val in range(row + 1, len(tree_grid[0])):
                if current_tree <= tree_grid[val][col]:
                    break
                if val == len(tree_grid[0]) - 1:  # we got to the edge, so the tree is visible from this direction
                    is_visible = True
            # west
            for val in range(col - 1, -1, -1):
                if current_tree <= tree_grid[row][val]:
                    break
                if val == 0:  # we got to the edge, so the tree is visible from this direction
                    is_visible = True

            # if the tree is visible from any direction, add it to the total
            # it could be visible from more than one, but we don't care about that.
            # We just if it's visible at all.
            if is_visible:
                visible += 1
    return visible


def evaluate_tree(tree_grid: List[List[int]], x: int, y: int) -> int:
    '''This calculates the scenic score for a given (x,y) coordinate'''
    up = calculate_view_distance(tree_grid, x, y, 0, -1)
    down = calculate_view_distance(tree_grid, x, y, 0, 1)
    left = calculate_view_distance(tree_grid, x, y, -1, 0)
    right = calculate_view_distance(tree_grid, x, y, 1, 0)

    return up * down * left * right


def calculate_view_distance(tree_grid: List[List[int]], x: int, y: int, x_direction: int, y_direction: int) -> int:
    '''This function walks in a direction and counts the number of trees visible.\n
    It stops either when a tree of the same (or taller) height is hit, or once it reaches the edge of the forest.
    '''
    x_len = len(tree_grid[0])
    y_len = len(tree_grid)
    current_tree_height = tree_grid[x][y]
    distance = 0
    x += x_direction
    y += y_direction
    while 0 <= x < x_len and 0 <= y < y_len:  # edge boundary check
        distance += 1
        if tree_grid[x][y] >= current_tree_height:  # taller tree check
            break
        x += x_direction
        y += y_direction
    return distance


def main(input_file: str):
    with open(input_file, 'r') as infile:
        tree_grid: List[List[int]] = []

        # build the tree line by line
        for line in infile:
            line = line.strip()
            row = []
            for val in line:
                row.append(int(val))
            tree_grid.append(row)

        print(f'part 1: {count_visible_trees(tree_grid)}')

        # iterate through the grid
        scores = []
        for x in range(len(tree_grid[0])):
            for y in range(len(tree_grid)):
                scores.append(evaluate_tree(tree_grid, x, y))
        scores.sort(reverse=True)
        print(f'part 2: {scores[0]}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
