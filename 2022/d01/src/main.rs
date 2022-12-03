use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input data!");
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let mut maxtotal = 0;
    let mut current = 0;

    for line in contents.lines() {
        if line.trim().is_empty() {
            if current > maxtotal {
                maxtotal = current;
            }
            current = 0;
        } else {
            current += line.trim().parse::<u32>().unwrap();
        }
    }

    println!("{}", maxtotal);
}

fn part2(contents: &str) {
    let mut heap = BinaryHeap::new();

    let mut total = 0;
    for line in contents.lines() {
        if line.trim().is_empty() {
            heap.push(total);
            total = 0;
        } else {
            total += line.trim().parse::<u32>().unwrap();
        }
    }

    let maxtotal = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{}", maxtotal);
}
