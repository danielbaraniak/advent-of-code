import re

XMAS = "XMAS"


def find_all_xmas(lines: list[str]) -> int:
    counter = 0

    counter += count_xmas(lines)

    transposed = transpose(lines)
    counter += count_xmas(transposed)

    staggered = transpose(staggered_transpose(lines))
    counter += count_xmas(staggered)

    staggered_b = transpose(staggered_back_transpose(lines))
    counter += count_xmas(staggered_b)

    return counter


def count_xmas(lines: list[str]) -> int:
    counter = 0
    for s in lines:
        counter += s.count(XMAS)
        counter += s[::-1].count(XMAS)
    return counter


def transpose(lines: list[str]) -> list[str]:
    return ["".join(x) for x in zip(*lines)]


def staggered_transpose(lines: list[str]) -> list[str]:
    result = []
    for i, s in enumerate(lines):
        n = i % len(s)
        result.append(s[n:] + s[:n])
    return result


def staggered_back_transpose(lines: list[str]) -> list[str]:
    result = []
    for i, s in enumerate(lines):
        n = i % len(s)
        result.append(s[-n:] + s[:-n])
    return result


def get_input(file_path: str) -> list[str]:
    with open(file_path) as f:
        return f.readlines()


input_data = get_input("data/input_04.txt")
print(find_all_xmas(input_data))


test_input = [
    "M.S",
    ".A.",
    "M.S",
]


def check_window(window: list[str]) -> bool:
    w1 = transpose(staggered_transpose(window))[0]
    w2 = transpose(staggered_back_transpose(window))[2]

    if (w1 == "MAS" or w1 == "SAM") and (w2 == "MAS" or w2 == "SAM"):
        return True


def find_x_mas(lines: list[str]) -> int:
    counter = 0

    for i, line in enumerate(lines[1:-1], start=1):
        for j, v in enumerate(line[1:-1], start=1):
            if v == "A":
                window = [lines[i-1][j-1:j+2], lines[i]
                          [j-1:j+2], lines[i+1][j-1:j+2]]
                if check_window(window):
                    counter += 1
    return counter


print(find_x_mas(input_data))
