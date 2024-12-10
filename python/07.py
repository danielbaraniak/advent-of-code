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
    return int(f"{a}{b}")


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


def test_operators_reverse(expected: int, values: list[int], operators):
    results = [expected]
    for a in values[::-1]:
        tmp_results = []
        for r in results:
            tmp = r - a
            tmp2, m = divmod(r, a)
            if m == 0 and tmp2 >= 0:
                tmp_results.append(tmp2)
            if tmp >= 0:
                tmp_results.append(tmp)
        if not tmp_results:
            return False
        results = tmp_results

    return 0 in results


def main():
    data = get_input()
    operators = (
        lambda a, b: a + b,
        lambda a, b: a * b,
        # concatenate
    )

    operators_reverse = (
        lambda a, b: a - b,
        lambda a, b: a / b,
        concatenate
    )

    calibration_result = 0
    for expected, values in data:
        if test_operators_reverse(expected, values, operators_reverse):
            calibration_result += expected
    print(calibration_result)

    calibration_result = 0
    for expected, values in data:
        if test_operators(expected, values, operators):
            calibration_result += expected
    print(calibration_result)


main()
