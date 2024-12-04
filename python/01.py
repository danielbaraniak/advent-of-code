def total_distance(list1, list2):
    return sum((abs(i-j) for i, j in zip(sorted(list1), sorted(list2))))


def get_input():
    with open("data\input_01.txt") as f:
        list1 = []
        list2 = []
        for line in f:
            nums = line.split()
            list1.append(int(nums[0]))
            list2.append(int(nums[1]))
    return list1, list2


def get_similarity_score(list1: list, list2: list) -> int:
    similarity_score = 0
    for n in list1:
        similarity_score += n * list2.count(n)
    return similarity_score


if __name__ == "__main__":
    list1, list2 = get_input()
    print(total_distance(list1, list2))
    print(get_similarity_score(list1, list2))
