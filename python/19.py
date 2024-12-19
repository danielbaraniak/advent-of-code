from tqdm import tqdm


def get_input(file_path: str) -> tuple[set[str], list[str]]:
    with open(file_path) as f:
        available, desired = f.read().strip().split("\n\n")
        available = [towel.strip() for towel in available.split(",")]
        desired = [towel.strip() for towel in desired.split("\n")]
        return available, desired


def is_possible(available: list[str], desired_towel: str) -> bool:
    for towel in available:
        if desired_towel == towel:
            return True
        if desired_towel.startswith(towel):
            part_possible = is_possible(
                available, desired_towel.removeprefix(towel))
            if part_possible:
                return True
    return False


def count_possible_combinations(available: list[str], desired_towel: str,  cache: dict[str, int]) -> bool:

    if desired_towel in cache:
        return cache[desired_towel]
    counter = 0

    if not desired_towel:
        return 1

    for i in range(1, len(desired_towel) + 1):
        towel_part = desired_towel[:i]
        if towel_part in available:
            counter += count_possible_combinations(
                available, desired_towel[i:], cache)

    cache[desired_towel] = counter
    return counter


def count_possible_towels(available: list[str], desired: list[str]) -> int:
    possible = 0
    for desired_towel in desired:
        if is_possible(available, desired_towel):
            possible += 1
    return possible


def count_possible_combiations_total(available: list[str], desired: list[str]) -> int:
    possible = 0
    cache = {}
    for desired_towel in tqdm(desired):
        if is_possible(available, desired_towel):
            possible += count_possible_combinations(
                available, desired_towel, cache=cache)
    return possible


def test():
    available, desired = get_input("rust/data/examples/19.txt")
    possible = count_possible_towels(available, desired)
    print(f"{available=}, {desired=}, {possible=}")
    assert possible == 6


def test2():
    available, desired = get_input("rust/data/examples/19.txt")
    possible = count_possible_combiations_total(available, desired)
    print(f"{available=}, {desired=}, {possible=}")
    assert possible == 16


def main():
    available, desired = get_input("rust/data/inputs/19.txt")
    possible = count_possible_towels(available, desired)
    print(possible)


def main2():
    available, desired = get_input("rust/data/inputs/19.txt")
    possible = count_possible_combiations_total(available, desired)
    print(possible)


test()
test2()
main()
main2()
