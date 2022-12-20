use regex::Regex;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;

#[macro_use]
extern crate derive_builder;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut monkeys = get_monkeys(contents);
    for _ in 0..20 {
        for _ in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();
            monkey.counter += monkey.items.len();
            for _ in 0..monkey.items.len() {
                let old = monkey.items.pop_front().unwrap();
                let (lhs, rhs) = monkey.operation.get_sides(old);
                let new = match monkey.operation.op {
                    Operator::Add => lhs + rhs,
                    Operator::Multiply => lhs * rhs,
                } / 3;
                let target = monkey.check.run(new);
                if monkey.index == target {
                    monkey.items.push_back(new);
                } else {
                    for m in monkeys.iter_mut() {
                        if m.index == target {
                            m.items.push_back(new);
                        }
                    }
                }
            }
            monkeys.push_back(monkey);
        }
    }
    println!("{}", product_largest_two(&monkeys));
}
fn part2(contents: &str) {
    let mut monkeys = get_monkeys(contents);
    let m: u128 = monkeys.iter().map(|m| m.check.param).product();
    for _ in 0..10000 {
        for _ in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();
            monkey.counter += monkey.items.len();
            for _ in 0..monkey.items.len() {
                let old = monkey.items.pop_front().unwrap();
                let (lhs, rhs) = monkey.operation.get_sides(old);
                let new = match monkey.operation.op {
                    Operator::Add => ((lhs % m) + (rhs % m)) % m,
                    Operator::Multiply => ((lhs % m) * (rhs % m)) % m,
                };
                let target = monkey.check.run(new);
                if monkey.index == target {
                    monkey.items.push_back(new);
                } else {
                    for m in monkeys.iter_mut() {
                        if m.index == target {
                            m.items.push_back(new);
                        }
                    }
                }
            }
            monkeys.push_back(monkey);
        }
    }
    println!("{}", product_largest_two(&monkeys));
}
#[derive(Builder, Clone)]
struct Monkey {
    counter: usize,
    index: u128,
    items: VecDeque<u128>,
    operation: Operation,
    check: Check,
}
#[derive(Clone)]
struct Operation {
    lhs: Option<u128>,
    rhs: Option<u128>,
    op: Operator,
}
impl Operation {
    fn get_sides(&self, default: u128) -> (u128, u128) {
        match (self.lhs, self.rhs) {
            (None, None) => (default, default),
            (None, Some(rhs)) => (default, rhs),
            (Some(lhs), None) => (lhs, default),
            (Some(lhs), Some(rhs)) => (lhs, rhs),
        }
    }
}
#[derive(Clone)]
struct Check {
    param: u128,
    lhs: u128,
    rhs: u128,
}
impl Check {
    fn run(&self, value: u128) -> u128 {
        match value % self.param == 0 {
            true => self.lhs,
            false => self.rhs,
        }
    }
}
#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}
fn create_check(param: &str, lhs: &str, rhs: &str) -> Check {
    Check {
        param: get_num(param).unwrap(),
        lhs: get_num(lhs).unwrap(),
        rhs: get_num(rhs).unwrap(),
    }
}
fn create_operation(lhs: &str, op: &str, rhs: &str) -> Operation {
    let op = match op {
        "+" => Operator::Add,
        "*" => Operator::Multiply,
        _ => unreachable!(),
    };
    let lhs = match lhs {
        "old" => None,
        _ => get_num(lhs),
    };
    let rhs = match rhs {
        "old" => None,
        _ => get_num(rhs),
    };
    Operation { lhs, rhs, op }
}
fn get_num(n: &str) -> Option<u128> {
    let trimmed = n.trim();
    let parsed: u128 = trimmed.parse().unwrap();
    Some(parsed)
}
fn get_monkeys(contents: &str) -> VecDeque<Monkey> {
    let re_index = Regex::new(r"^Monkey\s(\d+):$").unwrap();
    let re_items = Regex::new(r"^\s+Starting\sitems:\s(.*)$").unwrap();
    let re_operation = Regex::new(r"^\s+Operation:\snew\s=\s(.*)\s([\+\*])\s(.*)$").unwrap();
    let re_test = Regex::new(r"^\s+Test:\sdivisible\sby\s(.*)$").unwrap();
    let re_test_true = Regex::new(r"^\s+If\strue:\sthrow\sto\smonkey\s(\d+)$").unwrap();
    let re_test_false = Regex::new(r"^\s+If\sfalse:\sthrow\sto\smonkey\s(\d+)$").unwrap();

    let mut q = VecDeque::new();

    let mut builder = MonkeyBuilder::default();
    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        if re_index.is_match(line) {
            if let Ok(monkey) = builder.build() {
                q.push_back(monkey);
            }
            let cap = re_index.captures(line).unwrap();
            builder = MonkeyBuilder::default();
            builder.counter = Some(0);
            builder.index = get_num(&cap[1]);
        } else if re_items.is_match(line) {
            let cap = re_items.captures(line).unwrap();
            builder.items(
                (&cap[1])
                    .split(',')
                    .map(|e| e.trim().parse().unwrap())
                    .collect(),
            );
        } else if re_operation.is_match(line) {
            let cap = re_operation.captures(line).unwrap();
            builder.operation(create_operation(&cap[1], &cap[2], &cap[3]));
        } else if re_test.is_match(line) {
            let cap = re_test.captures(line).unwrap();
            let true_cap = re_test_true.captures(lines.next().unwrap()).unwrap();
            let false_cap = re_test_false.captures(lines.next().unwrap()).unwrap();
            builder.check(create_check(&cap[1], &true_cap[1], &false_cap[1]));
        }
    }
    if let Ok(monkey) = builder.build() {
        q.push_back(monkey);
    }
    q
}
fn product_largest_two(monkeys: &VecDeque<Monkey>) -> usize {
    let mut h = BinaryHeap::new();
    for m in monkeys.iter() {
        h.push(m.counter);
    }
    h.pop().unwrap() * h.pop().unwrap()
}
