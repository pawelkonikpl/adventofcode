import os

import requests as requests


def main():
    data = []
    with open("data.txt", "r", encoding='utf-8') as f:
        [data.append(int(line.strip())) for line in f.readlines()]

    prev = 0
    counter = 0
    for d in data:
        if d > prev:
            counter += 1
        prev = d
    print(counter -1)


if __name__ == '__main__':
    main()
