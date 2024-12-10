from collections import defaultdict
import itertools
import functools


def get_input(file_path: str) -> list[str]:
    with open(file_path) as f:
        return f.read().strip()


def range_sum(start, end):
    return ((start + end) * (end - start) / 2)


def defragment(data):
    memory = []
    file_id = 0
    for i, c in enumerate(data):
        if i % 2 == 0:
            memory += [file_id] * int(c)
            file_id += 1
        else:
            memory += [-1] * int(c)

    defrag_memory = []
    i = len(memory) - 1
    for j, c in enumerate(memory):
        if i <= j:
            break

        if c == -1:
            defrag_memory.append(memory[i])
            i -= 1
            while memory[i] == -1:
                i -= 1

        else:
            defrag_memory.append(c)

    checksum = 0
    for i, file_id in enumerate(defrag_memory):
        if file_id == -1:
            break
        checksum += i * file_id

    return checksum


def replace(file_id, size, available_size):
    return (file_id, size), (-1, available_size - size)


def main():
    data = get_input("rust/data/inputs/09.txt")
    checksum = defragment(data)

    print(checksum)


main()
