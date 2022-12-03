use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("read error");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut total = 0;
    for line in contents.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        total += find_common(&[a, b]);
    }
    println!("{}", total);
}
fn part2(contents: &str) {
    let mut lines = contents.lines();
    let mut total = 0;
    loop {
        let line1 = lines.next().unwrap_or("");
        if line1.is_empty() {
            break;
        }
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();
        total += find_common(&[line1, line2, line3]);
    }

    println!("{}", total);
}
fn find_common(parts: &[&str]) -> u32 {
    let mut table = [0; 53];
    let mut level = 1;
    for part in parts {
        for chr in part.chars() {
            let i = compute_index(&chr);
            if table[i] == level - 1 {
                table[i] += 1;
            }
        }
        level += 1;
    }

    table
        .into_iter()
        .enumerate()
        .filter(|(_, e)| *e == level - 1)
        .map(|(i, _)| i)
        .next()
        .unwrap() as u32
}
fn compute_index(chr: &char) -> usize {
    let base = chr.to_digit(36).unwrap() as i32;
    if chr.is_lowercase() {
        (base - 9) as usize
    } else {
        (base + 17) as usize
    }
}
