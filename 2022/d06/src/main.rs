use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let stream = contents.lines().last().unwrap();
    let mut q = VecDeque::with_capacity(3);

    let mut target = 0;
    for (i, chr) in stream.chars().enumerate() {
        if q.len() < 3 {
            q.push_back(chr);
        } else if q.contains(&chr) {
            q.pop_front();
            q.push_back(chr);
        } else {
            let mut v: Vec<&char> = q.iter().collect();
            v.sort();
            v.dedup();
            if v.len() == q.len() {
                target = i + 1;
                break;
            } else {
                q.pop_front();
                q.push_back(chr);
            }
        }
    }
    println!("{}", target);
}
fn part2(contents: &str) {
    let stream = contents.lines().last().unwrap();
    let mut q = VecDeque::with_capacity(13);

    let mut target = 0;
    for (i, chr) in stream.chars().enumerate() {
        if q.len() < 13 {
            q.push_back(chr);
        } else if q.contains(&chr) {
            q.pop_front();
            q.push_back(chr);
        } else {
            let mut v: Vec<&char> = q.iter().collect();
            v.sort();
            v.dedup();
            if v.len() == q.len() {
                target = i + 1;
                break;
            } else {
                q.pop_front();
                q.push_back(chr);
            }
        }
    }
    println!("{}", target);
}
