def prepare_data(line: str):
    data = []
    if not line == "":
        data = int(line)
    else:
        data = "-"
    return data


def main():
    data = []
    with open("data.txt", "r", encoding='utf-8') as f:
        [data.append(prepare_data(line.strip())) for line in f.readlines()]

    elfs = []

    for d in data:
        index = next((index for index, x in enumerate(data) if x == "-"), None)
        print("INDEX", index)
        if index:
            items = data[:index]
            elfs.append(sum(items))
            data = data[index+1:]

    print(sorted(elfs))

if __name__ == '__main__':
    main()
