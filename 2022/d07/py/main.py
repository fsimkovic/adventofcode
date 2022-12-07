from typing import NamedTuple, Dict, Optional, List
from pathlib import Path
def main() -> None:
    contents = Path("..", "input.txt").read_text()
    part1(contents)
    part2(contents)

def part1(contents: str) -> None:
    trie = FileSystem()
    reading_sizes = False

    for line in contents.splitlines():
        if line.startswith("$ cd"):
            reading_sizes = False
            trie.cd(line.split()[-1])

        if reading_sizes:
            size_or_dir, name = line.split()
            if size_or_dir.isdigit():
                trie.setsize(name, int(size_or_dir))

        if line.startswith("$ ls"):
            reading_sizes = True

    print(sum(node.size for node in trie.find(100_000)))


def part2(contents: str) -> None:
    trie = FileSystem()
    reading_sizes = False

    for line in contents.splitlines():
        if line.startswith("$ cd"):
            reading_sizes = False
            trie.cd(line.split()[-1])

        if reading_sizes:
            size_or_dir, name = line.split()
            if size_or_dir.isdigit():
                trie.setsize(name, int(size_or_dir))

        if line.startswith("$ ls"):
            reading_sizes = True
    
    empty = 70_000_000 - trie.root.size
    missing = 30_000_000 - empty

    unfiltered = (node.size for node in trie.find(70_000_000))
    sizes = filter(lambda s: s >= missing, unfiltered)
    print(min(sizes))


class Node:
    def __init__(self, name, size, parent, isleaf):
        self.name = name
        self.parent = parent
        self.children = {}
        self.isleaf = isleaf
        self._size = size

    def __repr__(self) -> str:
        return str(self.children)

    @property
    def size(self) -> int:
        if self._size is None:
            self._size = sum(n.size for n in self.children.values())
        return self._size



class FileSystem:
    def __init__(self) -> None:
        self.root = Node(None, None, None, False)
        self.current = self.root

    def __str__(self) -> str:
        return str(self.root)

    def cd(self, name: str) -> None:
        if name == "..":
            if self.current.parent is None:
                raise Exception
            self.current = self.current.parent
        else:
            self.current.children.setdefault(name, Node(name, None, self.current, False))
            self.current = self.current.children[name]

    def setsize(self, name: str, size: int) -> None:
        self.current.children.setdefault(name, Node(name, size, self.current, True))

    def find(self, threshold: int) -> List["Node"]:
        def recurse(node) -> List["Node"]:
            if node.isleaf:
                return []

            nodes = []
            for child in node.children.values():
                nodes.extend(recurse(child))
                if child.size <= threshold and not child.isleaf:
                    nodes.append(child)
            return nodes

        return recurse(self.root)




if __name__ == "__main__":
    main()
