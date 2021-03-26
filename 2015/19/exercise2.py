
import re
import queue
import collections

def read_data():
    with open("input1.txt") as fh:
        data = fh.readlines()
    return data

def get_replacements(data):
    out = []
    for e in data:
        k, v = e.split("=>")
        out.append((v.strip(), k.strip()))
    return out

def find(target, replacements):

    q = queue.PriorityQueue()
    q.put((len(target), target, 0))

    while not q.empty():
        _, target, steps = q.get()
        if target == "e":
            return steps

        for k, v in replacements:
            for match in re.finditer(k, target):
                new = target[:match.start()] + v + target[match.end():]
                q.put((len(new), new, steps + 1))
    return -1


if __name__ == "__main__":
    data = read_data()

    target = data.pop().strip()
    data.pop()  # throw away an empty line

    replacements = get_replacements(data)

    print(find(target, replacements))
