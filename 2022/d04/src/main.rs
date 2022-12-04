use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("failed to read input");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut count = 0;
    for line in contents.lines() {
        let (x1, y1, x2, y2) = unroll(line);
        if contain(x1, y1, x2, y2) {
            count += 1;
        }
    }
    println!("{}", count);
}
fn part2(contents: &str) {
    let mut count = 0;
    for line in contents.lines() {
        let (x1, y1, x2, y2) = unroll(line);
        if contain(x1, y1, x2, y2) || overlap(x1, y1, x2, y2) {
            count += 1;
        }
    }
    println!("{}", count);
}
fn unroll(s: &str) -> (u32, u32, u32, u32) {
    let (lhs, rhs) = s.split_once(',').unwrap();
    let (x1, y1) = split_by_underscore(lhs);
    let (x2, y2) = split_by_underscore(rhs);
    (x1, y1, x2, y2)
}
fn split_by_underscore(s: &str) -> (u32, u32) {
    let (lhs, rhs) = s.split_once('-').unwrap();
    let x: u32 = lhs.parse().unwrap();
    let y: u32 = rhs.parse().unwrap();
    (x, y)
}
fn contain(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    x1 <= x2 && y1 >= y2 || x2 <= x1 && y2 >= y1
}
fn overlap(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    x1 <= x2 && x2 <= y1 || x2 <= x1 && x1 <= y2
}
