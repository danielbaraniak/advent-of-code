input_file = "rust/data/inputs/2025/03.txt"


example_input = """987654321111111
811111111111119
234234234234278
818181911112111
"""


def read_input() -> str:
    with open(input_file) as f:
        return f.read()


def parse_input(data: str) -> list[list[int]]:
    lines = data.strip().splitlines()
    banks = []
    for l in lines:
        banks.append([int(i) for i in l])

    return banks


def part1(banks: list[list[int]]) -> int:
    return sum(find_highest_2(bank) for bank in banks)


def find_highest(s: list[int]) -> int:
    m1 = (0, 0)
    m2 = (1, 0)

    for i, n in enumerate(s[:-1]):
        if n > m1[0]:
            m1 = (n, i)

    for i, n in enumerate(s[m1[1]+1:]):
        if n > m2[0]:
            m2 = (n, i)
    return m1[0] * 10 + m2[0]


BATTERIES_COUNT = 12


def find_highest_2(s: list[int]) -> int:
    m = [(0, 0)] * BATTERIES_COUNT

    for i in range(BATTERIES_COUNT):
        start = m[i-1][1]+1 if i >= 1 else 0
        end = len(s) - BATTERIES_COUNT + i + 1

        for j, n in enumerate(s[start:end], start):
            if n > m[i][0]:
                m[i] = (n, j)

    result = "".join((str(n) for n, _ in m))
    return int(result)


def find_highest_two_pointers(s: list[int]) -> int:
    p1 = 0
    l = len(s)
    p2 = l - 1

    m = s[p1] * 10 + s[p2]

    while p1 < p2:
        while p2 - 1 > p1 and s[p2] <= s[p2 - 1]:
            p2 -= 1

        while p1 < p2 - 1 and s[p1] <= s[p1 + 1]:
            p1 += 1

        m = max(m, s[p1] * 10 + s[p2])

        if s[p2 - 1] < s[p1 + 1]:
            p2 -= 1
        else:
            p1 += 1

    print(m)

    return m


if __name__ == "__main__":
    data = read_input()
    ranges = parse_input(data)
    # ranges = parse_input(example_input)
    print(part1(ranges))
