from enum import Enum


def prepare_data(line: str):
    data = []
    for index, l in enumerate(line):
        data.append(int(l))
    return data


def main():
    data = []
    with open("data.txt", "r", encoding='ascii') as f:
        [data.append(prepare_data(line.strip())) for line in f.readlines()]

    print(data)
    winning_bit = [[0, 0] for d in data[0]]
    for result in data:
        for index, bit in enumerate(result):
            prev = winning_bit[index]
            prevValue = prev[bit]
            winning_bit[index][bit] = prevValue + 1

    print(winning_bit)
    gamma = ""
    epsilon = ""
    for rate in winning_bit:
        if rate[0] > rate[1]:
            gamma += "1"
            epsilon += "0"
        else:
            gamma += "0"
            epsilon += "1"

    print(int(gamma, 2) * int(epsilon, 2))


if __name__ == '__main__':
    main()
