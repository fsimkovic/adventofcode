use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("failed to read");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut container = Vec::new();
    for _ in 0..9 {
        container.push(Vec::new());
    }

    for line in contents.lines() {
        if line.trim().is_empty() {
            break;
        }
        for (i, cont) in container.iter_mut().enumerate().take(9) {
            let j = 1 + 4 * i;
            if line.chars().nth(j - 1).unwrap() == '[' {
                let chr = line.chars().nth(j).unwrap();
                cont.push(chr);
            }
        }
    }
    for cont in container.iter_mut().take(9) {
        cont.reverse();
    }

    for line in contents.lines() {
        if line.starts_with("move") {
            let mut count: usize = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            let x: usize = line.split_whitespace().nth(3).unwrap().parse().unwrap();
            let y: usize = line.split_whitespace().nth(5).unwrap().parse().unwrap();
            while count > 0 {
                let value = container[x - 1].pop().unwrap();
                container[y - 1].push(value);
                count -= 1;
            }
        }
    }
    print_peaks(&container);
}
fn part2(contents: &str) {
    let mut container = Vec::new();
    for _ in 0..9 {
        container.push(Vec::new());
    }

    for line in contents.lines() {
        if line.trim().is_empty() {
            break;
        }
        for (i, cont) in container.iter_mut().enumerate().take(9) {
            let j = 1 + 4 * i;
            if line.chars().nth(j - 1).unwrap() == '[' {
                let chr = line.chars().nth(j).unwrap();
                cont.push(chr);
            }
        }
    }
    for cont in container.iter_mut().take(9) {
        cont.reverse();
    }

    let mut tmp = Vec::new();
    for line in contents.lines() {
        if line.starts_with("move") {
            let mut count: usize = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            let x: usize = line.split_whitespace().nth(3).unwrap().parse().unwrap();
            let y: usize = line.split_whitespace().nth(5).unwrap().parse().unwrap();

            while count > 0 {
                let value = container[x - 1].pop().unwrap();
                tmp.push(value);
                count -= 1;
            }
            while !tmp.is_empty() {
                container[y - 1].push(tmp.pop().unwrap());
            }
        }
    }
    print_peaks(&container);
}
fn print_peaks(containers: &[Vec<char>]) {
    for cont in containers.iter().take(9) {
        if !cont.is_empty() {
            print!("{}", cont.last().unwrap());
        }
    }
    println!();
}
