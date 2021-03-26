
def read_data():
    with open("input1.txt") as fh:
        data = fh.readlines()
    return data

def get_replacements(data):
    out = []
    for e in data:
        k, v = e.split("=>")
        out.append((k.strip(), v.strip()))
    return out


if __name__ == "__main__":
    data = read_data()

    target = data.pop().strip()
    data.pop()  # throw away an empty line

    seen = set()
    replacements = get_replacements(data)
    while replacements:
        k, v = replacements.pop()

        for i in range(len(target) - len(k)):
            if target[i : i + len(k)] == k:
                seen.add(target[:i] + v + target[i + len(k):])


    print(len(seen))
