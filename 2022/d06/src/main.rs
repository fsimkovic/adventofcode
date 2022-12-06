use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let target = find(contents, 3);
    println!("{}", target);
}
fn part2(contents: &str) {
    let target = find(contents, 13);
    println!("{}", target);
}
fn find(contents: &str, nunique: usize) -> usize {
    let stream = contents.lines().last().unwrap();
    let mut q = VecDeque::with_capacity(nunique);

    for (i, chr) in stream.chars().enumerate() {
        if q.len() < nunique {
            q.push_back(chr);
        } else if q.contains(&chr) {
            q.pop_front();
            q.push_back(chr);
        } else {
            let mut v: Vec<&char> = q.iter().collect();
            v.sort();
            v.dedup();
            if v.len() == q.len() {
                return i + 1;
            }
            q.pop_front();
            q.push_back(chr);
        }
    }
    0
}
