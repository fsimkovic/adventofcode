from ast import literal_eval
from collections import namedtuple
from functools import cmp_to_key
from pathlib import Path
from typing import Any, List, Tuple


class Cmp:
    Lt = -1
    Eq = 0
    Gt = 1


def main() -> None:
    contents = Path("..", "input.txt").read_text()
    part1(contents)
    part2(contents)


def part1(contents: str) -> None:
    count = sum(i + 1 for i, (lhs, rhs) in enumerate(to_pairs(contents)) if compare(lhs, rhs) == Cmp.Lt)
    print(count)


def part2(contents: str) -> None:
    packets = [literal_eval(packet) for packet in contents.splitlines() if packet]
    div1 = [[2]]
    div2 = [[6]]
    packets += [div1, div2]
    packets.sort(key=cmp_to_key(compare))
    count = (packets.index(div1) + 1) * (packets.index(div2) + 1)
    print(count)


def to_pairs(contents: str) -> List[Tuple[Any, Any]]:
    lines = contents.splitlines()
    return [(literal_eval(lines[i]), literal_eval(lines[i + 1])) for i in range(0, len(lines), 3)]


def compare(lhs: Any, rhs: Any) -> int:
    if lhs == rhs:
        return Cmp.Eq
    if isinstance(lhs, int) and isinstance(rhs, int):
        return Cmp.Lt if lhs < rhs else Cmp.Gt
    if isinstance(lhs, int):
        return compare([lhs], rhs)
    if isinstance(rhs, int):
        return compare(lhs, [rhs])
    if lhs and rhs:
        res = compare(lhs[0], rhs[0])
        if res == Cmp.Eq:
            return compare(lhs[1:], rhs[1:])
        return res
    return Cmp.Gt if lhs else Cmp.Lt


if __name__ == "__main__":
    main()
