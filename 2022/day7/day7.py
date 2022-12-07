#!/usr/bin/env python3

from __future__ import annotations

from sys import argv, exit, stderr
from collections import defaultdict
from typing import Union, List, Tuple
from dataclasses import dataclass, field

expected_args = 2


@dataclass
class File:
    """Class that houses a file, as determined by the input"""
    name: str
    size: int


@dataclass
class Dir:
    """Class that houses a directory"""
    name: str
    parent_name: str = None
    files: dict = field(default_factory=defaultdict)

    def files_in_dir(self) -> List[Union[File, Dir]]:
        '''Gives all files living in a current directory. subdirectories are included'''
        result = [self]
        for file in self.files.values():
            if isinstance(file, Dir):
                result.extend(file.files_in_dir())
            else:
                result.append(file)
        return result

    def total_space_used(self) -> int:
        '''Unlike `size` below, this calculates the total space used (so it double counts)'''
        return sum(file.size for file in self.files_in_dir() if isinstance(file, File))


class InvalidDirectoryException(Exception):
    '''Raised if you try to `cd` into a non-existent directory'''
    def __init__(self, message):
        super().__init__(message)


def size(en: Union[File, Dir]) -> int:
    '''Gets the size of a given File or Directory, including subdirectories'''
    def loop(total: int, left: List[Union[File, Dir]]) -> int:
        '''This function uses total as an accumulator to be tail-recursive'''
        rest = left[1:]
        if not left:
            return total
        match left[0]:
            case File(_, sz):
                return loop(total + sz, rest)
            case Dir(_, _, fls):
                rest.extend(fls.values())
                return loop(total, rest)
    return loop(0, [en])


def process_lines(input_file: str) -> List[Tuple[str, List[str]]]:
    commands = []
    with open(input_file, 'r') as infile:
        for line in infile:
            if line.startswith("$"):
                commands.append((line[1:].strip(), []))
            else:
                commands[-1][1].append(line.strip('\n'))
    return commands


def build_directory_structure(commands: List[Tuple[str, List[str]]]) -> Dir:
    root_dir = Dir(name="/")
    working_directory = root_dir
    for command, outs in commands:
        if command.startswith("cd"):  # outs is not used in this case
            destination = command.split()[1]
            match destination:
                case "/":
                    working_directory = root_dir
                case "..":
                    working_directory = working_directory.parent_name
                case _:  # this means we have a target to go to
                    if destination in working_directory.files:
                        working_directory = working_directory.files[destination]
                    else:  # raise an error, invalid directory given
                        raise InvalidDirectoryException(f'No directory {destination} in {working_directory}')
        elif command.startswith("ls"):
            for out in outs:
                size, name = out.split()  # funny quirk, size could be the word 'dir' meaning "make a new directory"
                if size == 'dir':
                    working_directory.files[name] = Dir(name, working_directory)
                else:
                    working_directory.files[name] = File(name, int(size))
    return root_dir


def main(input_file: str):

    commands = process_lines(input_file)
    try:
        root_dir = build_directory_structure(commands)
    except InvalidDirectoryException as e:
        print(e)
        exit(1)

    total = 0
    total_disk_space = 70_000_000

    for file in root_dir.files_in_dir():
        if isinstance(file, Dir):
            sz = file.total_space_used()
            if sz <= 100_000:
                total += sz
    print(f'Part 1 total: {total}')

    free_space = total_disk_space - root_dir.total_space_used()

    # create a list of all folders we could delete to create enough update space
    can_be_freed = [
        file for file in root_dir.files_in_dir()
        if isinstance(file, Dir) and file.total_space_used() + free_space >= 30_000_000
    ]

    # find the size of the smallest folder that fits the criterion
    print(f'Part 2: {min(file.total_space_used() for file in can_be_freed)}')


if __name__ == "__main__":
    if len(argv) != expected_args:
        print(f'ERROR: expected {expected_args - 1} arguments (input_file)', file=stderr)
        exit(1)
    main(argv[1])
