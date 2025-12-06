from operator import itemgetter
from bisect import bisect_left
input_file = "rust/data/inputs/05.txt"
example_input = "rust/data/examples/05.txt"


def read_input(path: str) -> str:
    with open(path) as f:
        return f.read()


def parse_input(data: str) -> list[list[int]]:
    ranges_ingrediens = data.strip().split("\n\n")
    ranges = ranges_ingrediens[0].strip().splitlines()
    ranges = [range_str.split("-") for range_str in ranges]
    ranges = [(int(r[0]), int(r[1])) for r in ranges]

    ingrediens = [int(i) for i in ranges_ingrediens[1].strip().splitlines()]

    return (ranges, ingrediens)


def merge_ranges(ranges: list[tuple[int, int]]) -> int:
    ranges = sorted(ranges)
    merged_ranages = []
    current = ranges[0]
    for s, e in ranges[1:]:
        if e <= current[1]:
            continue
        if s <= current[1] <= e:
            current = (current[0], e)
        elif s > current[1]:
            merged_ranages.append(current)
            current = s, e

    merged_ranages.append(current)

    return merged_ranages


def part1(ranges: list[tuple[int, int]], ingredients: list[int]) -> int:
    ranges = merge_ranges(ranges)

    counter = 0
    for ingredient in ingredients:
        i = bisect_left(ranges, ingredient, key=itemgetter(1))
        if i >= len(ranges):
            continue
        s, e = ranges[i]
        if s <= ingredient <= e:
            counter += 1

    return counter


def part2(ranges: list[tuple[int, int]], _: list[int]) -> int:
    ranges = merge_ranges(ranges)

    return sum(e - s + 1 for s, e in ranges)


if __name__ == "__main__":
    path = example_input
    path = input_file
    data = read_input(path)
    parsed_data = parse_input(data)

    print(part1(*parsed_data))
    print(part2(*parsed_data))
