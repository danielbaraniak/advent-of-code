input_file = "rust/data/examples/01.txt"


example_input = [
    -68,
    -30,
    48,
    +2,
    -2,
    -5,
    60,
    -55,
    -1,
    -99,
    14,
    -82
]


def read_input() -> list[int]:
    with open(input_file) as f:
        numbers = []
        for line in f.readlines():
            line = line.strip()
            if line == "":
                continue
            if line[0] == "L":
                numbers.append(-int(line[1:]))
            else:
                numbers.append(int(line[1:]))
    return numbers


def part1(numbers: list[int]) -> int:
    current = 50

    clip = [x % 100 for x in numbers]

    result = 0

    for x in clip:
        _, current = divmod(x + current, 100)
        if current == 0:
            result += 1

    return result


def part2(numbers: list[int]) -> int:
    current = 50
    result = 0

    r = 0
    last = 50
    arr = []
    for y in numbers:
        x, last = divmod(y + last, 100)
        arr.append((x, last, y))

    r = 0
    current = 50
    last_zero_pass = 0
    last = -1
    last_y = 0
    for zero_pass, current, y in arr:
        print(zero_pass, current, y)
        if last_y > 0 and y < 0 and last == 0:
            r -= 1
        if last_y < 0 and y > 0 and last == 0:
            r += 1
        r += abs(zero_pass)
        last = current
        last_zero_pass = zero_pass
        last_y = y

    return r


if __name__ == "__main__":

    print(divmod(-66, 100))

    numbers = read_input()
    print("Part 1:", part1(numbers))
    print("Part 2:", part2(numbers))
