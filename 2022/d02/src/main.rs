use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("failed to read");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut total = 0;

    for line in contents.lines() {
        let mut it = line.split_whitespace();
        let s1 = Shape::from(it.next().unwrap());
        let s2 = Shape::from(it.next().unwrap());
        total += s1.fight(&s2);
    }

    println!("{}", total);
}
fn part2(contents: &str) {
    let mut total = 0;

    for line in contents.lines() {
        let mut it = line.split_whitespace();
        let s1 = Shape::from(it.next().unwrap());
        let s2 = s1.resolve(it.next().unwrap());
        total += s1.fight(&s2);
    }

    println!("{}", total);
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from(chr: &str) -> Self {
        match chr {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
    fn resolve(&self, chr: &str) -> Shape {
        match (self, chr) {
            (Shape::Rock, "X") => Shape::Scissors,
            (Shape::Rock, "Y") => Shape::Rock,
            (Shape::Rock, "Z") => Shape::Paper,
            (Shape::Paper, "X") => Shape::Rock,
            (Shape::Paper, "Y") => Shape::Paper,
            (Shape::Paper, "Z") => Shape::Scissors,
            (Shape::Scissors, "X") => Shape::Paper,
            (Shape::Scissors, "Y") => Shape::Scissors,
            (Shape::Scissors, "Z") => Shape::Rock,
            _ => unreachable!(),
        }
    }
    fn fight(&self, other: &Shape) -> u32 {
        let r1 = match (other, self) {
            (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => 3,
            _ => 0,
        };
        let r2 = match other {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
        r1 + r2
    }
}
