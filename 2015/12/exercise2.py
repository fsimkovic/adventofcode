import json

def drop_red(data):

    if isinstance(data, list):
        for v in data:
            drop_red(v)

    elif isinstance(data, dict):
        w_red = False
        for k, v in data.items():
            if k == "red" or v == "red":
                w_red = True

        if w_red:
            for k in list(data):
                data.pop(k)

        for v in data.values():
            drop_red(v)


def count(data):
    if isinstance(data, int):
        return data
    elif isinstance(data, str):
        return 0
    elif isinstance(data, (list, tuple)):
        total = 0
        for v in data:
            total += count(v)
        return total
    elif isinstance(data, dict):
        total = 0
        for v in data.items():
            total += count(v)
        return total
    else:
        raise Exception(type(data))




if __name__ == "__main__":
    with open("input1.txt") as fh:
        data = json.loads(fh.read())
    drop_red(data)
    print(count(data))
