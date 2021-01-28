
import collections
import dataclasses
import enum

class OpCode(enum.Enum):
    AND = enum.auto()
    OR = enum.auto()
    RSHIFT = enum.auto()
    LSHIFT = enum.auto()
    NOT = enum.auto()
    MAGIC = enum.auto()

@dataclasses.dataclass
class Instruction:
    opcode: OpCode
    left: str
    right: str
    target: str

    @property
    def l_is_digit(self):
        return self.left.isnumeric()

    @property
    def r_is_digit(self):
        return self.right.isnumeric()


def digest(line):
    parts = line.split()
    if len(parts) == 3:
        return Instruction(OpCode.MAGIC, parts[0], None, parts[-1])
    elif parts[0] == "NOT":
        return Instruction(OpCode.NOT, None, parts[1], parts[-1])
    return Instruction(OpCode[parts[1]], parts[0], parts[2], parts[-1])


def process(memory, op):
    l_int = memory.get(op.left, int(op.left) if op.l_is_digit else None)
    r_int = memory.get(op.right, int(op.right) if op.r_is_digit else None)

    if l_int is None or r_int is None:
        pass
    elif op.opcode == OpCode.AND:
        memory[op.target] = l_int & r_int
    elif op.opcode == OpCode.OR:
        memory[op.target] = l_int | r_int
    elif op.opcode == OpCode.LSHIFT:
        memory[op.target] = l_int << r_int
    elif op.opcode == OpCode.RSHIFT:
        memory[op.target] = l_int >> r_int
    
def iterate(memory, instructions):
    for op in instructions:
        if op.target in memory:
            continue
        elif op.opcode == OpCode.MAGIC:
            if op.l_is_digit:
                memory[op.target] = int(op.left)
            elif op.left in memory:
                memory[op.target] = memory[op.left]
        elif op.opcode == OpCode.NOT:
            if op.r_is_digit:
                memory[op.target] = ~(int(op.right))
            elif op.right in memory:
                memory[op.target] = ~(memory[op.right])
        else:
            process(memory, op)

def main():
    with open("input1.txt") as fh:
        instructions = list(map(digest, fh.readlines()))

    memory = {}
    while "a" not in memory:
        iterate(memory, instructions)
    print("Value 'a':", memory["a"])



if __name__ == "__main__":
    main()
