import time


max_length = len("mul(999,999)")
min_length = len("mul(1,1)")


def parse(s: str):
    valid_numbers = []
    i = 0
    s += " " * (max_length - min_length)
    for i in range(len(s)-max_length):
        maybe_mul = s[i:i+max_length]
        if not maybe_mul.startswith("mul("):
            continue
        bracket_index = maybe_mul.find(")")
        if bracket_index == -1:
            continue
        maybe_nums = maybe_mul[4:bracket_index].split(",", 1)
        if len(maybe_nums) != 2:
            continue
        try:
            nums = tuple(int(x) for x in maybe_nums)
            valid_numbers.append(nums)
        except ValueError:
            continue
    return valid_numbers


def parse_find_with_do_dont(s: str):
    valid_numbers = []
    i = 0
    s += " " * (max_length - min_length)
    limit = len(s) - max_length
    while i < limit:
        mul_position = s[i:].find("mul(")
        dont_position = s[i:].find("don't()")
        if dont_position != -1 and dont_position < mul_position:
            i += dont_position + 7
            do_position = s[i:].find("do()")
            if do_position == -1:
                break
            i += do_position + 4
            continue
        if mul_position == -1:
            break
        i += mul_position
        maybe_mul = s[i:i+max_length]
        bracket_index = maybe_mul.find(")")
        if bracket_index == -1:
            i += 4
            continue
        maybe_nums = maybe_mul[4:bracket_index].split(",", 1)
        if len(maybe_nums) != 2:
            i += bracket_index + 1
            continue
        try:
            nums = tuple(int(x) for x in maybe_nums)
            valid_numbers.append(nums)
            i += bracket_index + 1
        except ValueError:
            i += bracket_index + 1
            continue
    return valid_numbers


def parse_input(file_path: str):
    valid_numbers = []
    with open(file_path) as f:
        s = f.read()
        valid_numbers = parse(s)
        multiplied_numbers = [x*y for x, y in valid_numbers]
        return sum(multiplied_numbers)


def parse_input_find_with_do_dont(file_path: str):
    valid_numbers = []
    with open(file_path) as f:
        s = f.read()
        valid_numbers = parse_find_with_do_dont(s)
        multiplied_numbers = [x*y for x, y in valid_numbers]
        return sum(multiplied_numbers)


print(parse_input("data/input_03.txt"))
print(parse_input_find_with_do_dont("data/input_03.txt"))
