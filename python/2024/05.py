from collections import defaultdict


def get_input():
    orders = []
    updates = []
    with open('data/input_05.txt') as f:

        while line := f.readline().strip():
            orders.append(tuple(int(n) for n in line.split("|")))
        while line := f.readline().strip():
            updates.append([int(n) for n in line.split(",")])

    return orders, updates


def build_lookup(orders) -> list:
    pre_to_post = defaultdict(list)
    for pre, post in sorted(orders):
        pre_to_post[pre].append(post)

    return pre_to_post


def validate_ordering(update, pre_to_post) -> bool:
    for i, n in enumerate(update):
        for pre in update[:i]:
            if pre in pre_to_post[n]:
                return False
    return True


def sort_pages(update, pre_to_post):
    result = []
    for page in update:
        for j, r in enumerate(result):
            if r in pre_to_post[page]:
                result.insert(j, page)
                break
        else:
            result.append(page)
    return result


def main():
    orders, updates = get_input()
    pre_to_post = build_lookup(orders)
    sum_middle = 0
    counter = 0
    sum_middle_of_sorted = 0
    for update in updates:
        if validate_ordering(update, pre_to_post):
            counter += 1
            sum_middle += update[len(update) // 2]
        else:
            sum_middle_of_sorted += sort_pages(update,
                                               pre_to_post)[len(update) // 2]
    print(f"{counter=}, {sum_middle=}, {len(updates)=}, {sum_middle_of_sorted=}")


main()
