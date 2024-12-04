def is_pair_safe(n1, n2, previous_diff):
    diff = n2 - n1
    if 1 <= diff <= 3 and previous_diff >= 0:
        return True, diff
    if -3 <= diff <= -1 and previous_diff <= 0:
        return True, diff
    return False, diff


def is_record_safe(record: list) -> bool:
    previous_diff = 0
    for i, n in enumerate(record[1:], start=1):
        is_safe, diff = is_pair_safe(record[i-1], n, previous_diff)
        if not is_safe:
            return False
        previous_diff = diff
    return True


def try_with_dampener(record: list) -> bool:
    if is_record_safe(record[1:]):
        return True
    previous_diff = 0
    for i in range(1, len(record)):
        is_safe, diff = is_pair_safe(record[i-1], record[i], previous_diff)
        if not is_safe:
            return is_record_safe(record[:i] + record[i+1:]) or is_record_safe(record[:i-1] + record[i:])
        previous_diff = diff
    return True


def process_input(file_path: str) -> int:
    safe_counter = 0
    with open(file_path) as f:
        for line in f:
            record = [int(x) for x in line.split()]
            if is_record_safe(record):
                safe_counter += 1
    return safe_counter


def process_input_dampener(file_path: str) -> int:
    safe_counter = 0
    with open(file_path) as f:
        for line in f:
            record = [int(x) for x in line.split()]
            if try_with_dampener(record):
                safe_counter += 1
    return safe_counter


print(process_input("data/input_02.txt"))
print(process_input_dampener("data/input_02.txt"))
