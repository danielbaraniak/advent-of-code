from itertools import combinations


input_file = "rust/data/inputs/2025/02.txt"


example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"


def read_input() -> str:
    with open(input_file) as f:
        return f.read().strip()


def parse_input(data: str) -> list[tuple[int, int]]:
    ranges = []
    for part in data.split(","):
        a, b = part.split("-")
        ranges.append((int(a), int(b)))
    return ranges


def part1(ranges: list[tuple[int, int]], is_invalid_predicate: callable) -> int:
    count = 0
    for a, b in ranges:
        for x in range(a, b + 1):
            if is_invalid_predicate(x):
                count += x
    return count


def is_invalid_2(x: int) -> bool:
    s = str(x)
    len_s = len(s)

    for size in range(1, len_s + 1):
        if len_s % size != 0:
            # print(f"skipped: {x} size={size}")
            continue

        chunks_to_check = [s[i:i+size] for i in range(0, len_s, size)]
        # print(f"checking: {x} size={size} chunks={chunks_to_check}")
        if len(chunks_to_check) > 1 and all(chunk == chunks_to_check[0] for chunk in chunks_to_check[1:]):
            # print(f"invalid_2: {x} size={size}")
            return True
    return False


def is_invalid(x: int) -> bool:
    s = str(x)
    len_s = len(s)

    if len_s % 2 != 0:
        return False
    if s[:len_s//2] == s[len_s//2:]:
        return True
    return False


if __name__ == "__main__":
    data = read_input()
    ranges = parse_input(data)
    # ranges = parse_input(example_input)
    print(part1(ranges, is_invalid_2))
