def get_input():
    with open("data/input_07.txt") as f:
        data = []
        for line in f:
            result, values = line.split(":", 1)
            result = int(result)
            values = [int(v) for v in values.split()]
            data.append((result, values))
    return data


def concatenate(a: int, b: int) -> int:
    ten_power = 10
    while True:
        if b >= ten_power:
            ten_power *= 10
        else:
            return a * ten_power + b


def test_operators(expected: int, values: list[int], operators):
    results = [values[0]]
    for n in values[1:]:
        results = [
            f(r, n)
            for f in operators
            for r in results
            if r <= expected
        ]

    return expected in results


def main():
    data = get_input()
    calibration_result = 0

    operators = (
        lambda a, b: a + b,
        lambda a, b: a * b,
        concatenate
    )

    for expected, values in data:
        if test_operators(expected, values, operators):
            calibration_result += expected
    print(calibration_result)


main()
