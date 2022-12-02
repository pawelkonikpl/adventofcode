from enum import Enum


class Direction(Enum):
    forward = "forward"
    down = "down"
    up = "up"


def prepare_data(line: str):
    line = line.strip()
    direction, amount = line.split(" ")
    if Direction[direction] == Direction.up:
        return dict(direction=Direction[direction], amount=-int(amount))
    return dict(direction=Direction[direction], amount=int(amount))


def main():
    data = []
    with open("data.txt", "r", encoding='utf-8') as f:
        [data.append(prepare_data(line)) for line in f.readlines()]

    horizontal = 0
    depth = 0

    for d in data:
        if d["direction"] == Direction.forward:
            horizontal += d["amount"]
        if d["direction"] == Direction.down:
            depth += d["amount"]
        if d["direction"] == Direction.up:
            depth += d["amount"]

    print(f"Depth {depth}, horizontal {horizontal}")
    print(f"Result ", depth * horizontal)


if __name__ == '__main__':
    main()
