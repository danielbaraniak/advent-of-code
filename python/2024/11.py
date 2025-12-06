def get_input(file_path: str) -> list[str]:
    with open(file_path) as f:
        return [int(x) for x in f.read().strip().split()]


def process_stones(stones: list[int]):
    for stone in stones:
        stone_str = str(stone)
        if stone == 0:
            yield 1
        elif len(stone_str) % 2 == 0:
            ls = stone_str[:len(stone_str) // 2]
            rs = stone_str[len(stone_str) // 2:]
            yield from (int(ls), int(rs))
        else:
            yield stone * 2024


def process(stones: list[int]) -> list[int]:
    for _ in range(35):
        stones = process_stones(stones)

    return stones


def main():
    initial = get_input("rust/data/inputs/11.txt")
    stones = process(initial)
    counter = sum(1 for _ in stones)
    print(counter)


example = [125, 17]


def test():
    initial = example
    stones = process(initial)
    counter = sum(1 for _ in stones)
    assert counter == 55312
    print("Passed")


test()
