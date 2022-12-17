use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    let instructions: Vec<Instruction> = contents.lines().map(Instruction::from_string).collect();
    part1(&instructions);
    part2(&instructions);
}
fn part1(instructions: &[Instruction]) {
    let mut register = 1;
    let mut clock_cycle = 1;
    let mut signal_strength = 0;

    for instruction in instructions {
        let cycles = match instruction {
            Instruction::Noop(cycles) => *cycles,
            Instruction::Addx(cycles, _) => *cycles,
        };

        for _ in 0..cycles {
            if (clock_cycle - 20) % 40 == 0 {
                signal_strength += clock_cycle * register;
            }
            clock_cycle += 1;
        }
        if let Instruction::Addx(_, value) = instruction {
            register += value;
        }
    }
    println!("{:?}", signal_strength);
}
fn part2(instructions: &[Instruction]) {
    let mut register = 1;
    let mut clock_cycle = 0;
    let mut pixel = 0;

    for instruction in instructions {
        let cycles = match instruction {
            Instruction::Noop(cycles) => *cycles,
            Instruction::Addx(cycles, _) => *cycles,
        };

        for _ in 0..cycles {
            if clock_cycle % 40 == 0 {
                pixel = 0;
                println!();
            }
            if pixel == register - 1 || pixel == register || pixel == register + 1 {
                print!("#");
            } else {
                print!(".");
            }
            pixel += 1;
            clock_cycle += 1;
        }
        if let Instruction::Addx(_, value) = instruction {
            register += value;
        }
    }
    println!();
}

enum Instruction {
    Noop(u32),
    Addx(u32, i32),
}
impl Instruction {
    fn from_string(line: &str) -> Instruction {
        if line.starts_with("noop") {
            Instruction::Noop(1)
        } else {
            let (_, value) = line.split_once(' ').unwrap();
            Instruction::Addx(2, value.parse().unwrap())
        }
    }
}
