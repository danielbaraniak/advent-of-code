from operator import itemgetter
from bisect import bisect_left
from math import prod

input_file = "rust/data/inputs/06.txt"
example_input = "rust/data/examples/06.txt"


def read_input(path: str) -> str:
    with open(path) as f:
        return f.read()


def parse_input(data: str) -> list[list[int]]:
    lines = data.strip().splitlines()
    data = [l.split() for l in lines]

    return data


def parse_input_2(data: str) -> list[list[int]]:
    lines = data.strip().splitlines()
    operations = lines[-1].split()

    max_len = max(len(line) for line in lines[:-1])
    lines = [line.ljust(max_len) for line in lines]

    arguments = [list(line) for line in lines[:-1]]

    all_args = []
    all_args.append([])
    i = 0
    for column in zip(*arguments):
        number = "".join(column).strip()
        if number:
            all_args[i].append(int(number))
        else:
            all_args.append([])
            i += 1

    return all_args, operations


def part1(data: list[str]) -> int:
    data = parse_input(data)
    total = 0

    for d in zip(*data):
        args = (int(a) for a in d[:-1])
        operation = d[-1]

        match operation:
            case "+": total += sum(args)
            case "*": total += prod(args)

    return total


def part2(data: list[str]) -> int:
    arguments, operations = parse_input_2(data)
    total = 0

    for args, operation in zip(arguments, operations):
        match operation:
            case "+": total += sum(args)
            case "*": total += prod(args)

    return total


if __name__ == "__main__":
    path = example_input
    path = input_file
    data = read_input(path)

    print(part1(data))
    print(part2(data))
