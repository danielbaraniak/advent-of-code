from tqdm import tqdm


class Position:
    def __init__(self, map):
        self.map = map
        self.coordinates = self.get_start(map)
        self.direction = (-1, 0)
        self.visited_positions = {(self.coordinates, self.direction)}

    def get_start(self, map):
        for x, row in enumerate(map):
            for y, char in enumerate(row):
                if char == "^":
                    return x, y

    def rotate(self):
        x, y = self.direction
        self.direction = y, -x

    def next_position(self):
        x, y = self.coordinates
        dx, dy = self.direction
        return x + dx, y + dy

    def is_in_bounds(self, coordinates):
        return 0 <= coordinates[0] < len(self.map) and 0 <= coordinates[1] < len(self.map[0])

    def simulate(self) -> tuple[any, bool]:
        next_coordinates = self.next_position()
        while True:
            if not self.is_in_bounds(next_coordinates):
                return self.visited_positions, False
            while self.map[next_coordinates[0]][next_coordinates[1]] == "#":
                self.rotate()
                next_coordinates = self.next_position()
            self.coordinates = next_coordinates
            if (self.coordinates, self.direction) in self.visited_positions:
                return None, True
            self.visited_positions.add((self.coordinates, self.direction))
            next_coordinates = self.next_position()


def get_input(file_path: str) -> list[str]:
    with open(file_path) as f:
        return [list(line.strip()) for line in f.readlines()]


def deep_copy(input):
    return [x.copy() for x in input]


def try_brutforce():
    loop_counter = 0
    for i, line in enumerate(input):
        for j, char in enumerate(line):
            if char == ".":
                tmp_input = deep_copy(input)
                tmp_input[i][j] = "#"
                position = Position(tmp_input)
                _, loop = position.simulate()
                print(
                    f"\r{loop_counter=:4d}, {(i*len(input[0]) + j) * 100 / (len(input) * len(input[0])):2f}%", end="")
                if loop:
                    loop_counter += 1
    return loop_counter


def try_blocking_visited(visited_positions):
    loop_counter = 0
    for (i, j) in tqdm(visited_positions):
        tmp_input = deep_copy(input)
        tmp_input[i][j] = "#"
        position = Position(tmp_input)
        _, loop = position.simulate()
        if loop:
            loop_counter += 1

    return loop_counter


input = get_input("data/input_06.txt")
position = Position(input)


visited_positions, _ = position.simulate()
visited_positions = {p[0] for p in visited_positions}

print(len(visited_positions))

visited_positions.remove(position.get_start(input))


print(try_blocking_visited(visited_positions))
