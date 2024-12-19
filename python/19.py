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


def count_possible_towels(available: list[str], desired: list[str]) -> int:
    possible = 0
    for desired_towel in desired:
        if is_possible(available, desired_towel):
            possible += 1
    return possible


def test():
    available, desired = get_input("rust/data/examples/19.txt")
    possible = count_possible_towels(available, desired)
    print(f"{available=}, {desired=}, {possible=}")
    assert possible == 6


def main():
    available, desired = get_input("rust/data/inputs/19.txt")
    possible = count_possible_towels(available, desired)
    print(possible)


test()
main()
