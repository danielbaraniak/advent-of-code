from collections import defaultdict
import itertools
import functools


def get_input(file_path: str) -> list[str]:
    with open(file_path) as f:
        return f.read().splitlines()


def antenna_positions(map: list[str]) -> dict:
    antennas = defaultdict(list)

    for i, row in enumerate(map):
        for j, cell in enumerate(row):
            if cell == ".":
                continue
            antennas[cell].append((i, j))
    return antennas


def is_in_bounds(range_row: slice, range_col: slice, node: tuple) -> bool:
    row, col = node
    return range_row.start <= row < range_row.stop and range_col.start <= col < range_col.stop


def get_pair_antinodes(first: tuple, second: tuple) -> tuple:
    dx = first[0] - second[0]
    dy = first[1] - second[1]
    antinodes = ((first[0] + dx * n, first[1] + dy * n)
                 for n in itertools.count())
    return antinodes


def get_frequency_antinodes(antennas: list[tuple], in_bounds) -> set:
    antinodes = set()
    for first, second in itertools.permutations(antennas, 2):
        for node in get_pair_antinodes(first, second):
            if not in_bounds(node=node):
                break
            antinodes.add(node)

    return antinodes


def main():
    antenna_map = get_input("data/input_08.txt")
    antennas = antenna_positions(antenna_map)
    all_nodes = set()

    range_row = slice(0, len(antenna_map))
    range_col = slice(0, len(antenna_map[0]))

    bounds_predicate = functools.partial(
        is_in_bounds,
        range_col=range_col,
        range_row=range_row
    )

    for positions in antennas.values():
        all_nodes |= get_frequency_antinodes(positions, bounds_predicate)

    print(len(all_nodes))


main()
