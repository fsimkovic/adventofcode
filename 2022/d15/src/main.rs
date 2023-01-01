use std::{collections::HashSet, fs, ops::RangeInclusive};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut positions = HashSet::new();
    let pairs = read_text(contents);
    let row = 2000000;
    pairs.iter().for_each(|p| {
        let dist = p.dist();
        for i in -dist..dist + 1 {
            let marker = Position::new(p.sensor.x + i, row);
            if p.sensor.dist(&marker) <= dist {
                positions.insert(marker);
            }
        }
    });
    pairs.iter().for_each(|p| {
        positions.remove(&p.sensor);
        positions.remove(&p.beacon);
    });
    println!("{}", positions.len());
}
fn part2(contents: &str) {
    const BOUND: i32 = 4000000;
    const FACT: usize = 4000000;
    let pairs = read_text(contents);
    for y in 0..=BOUND {
        let mut raw_ranges: Vec<RangeInclusive<usize>> = pairs
            .iter()
            .filter_map(|p| p.row_range(y, 0, BOUND))
            .collect();
        raw_ranges.sort_by(|a, b| b.start().cmp(a.start()));
        let ranges = collapse_ranges(&mut raw_ranges);
        if ranges.len() > 1 {
            let x = ranges[0].end() + 1;
            println!("{:?}", x * FACT + y as usize);
            break;
        }
    }
}
fn collapse_ranges(ranges: &mut Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut collapsed = vec![ranges.pop().unwrap()];
    while !ranges.is_empty() {
        let prev = collapsed.pop().unwrap();
        let curr = ranges.pop().unwrap();
        if prev.end() + 1 == *curr.start() {
            collapsed.push(*prev.start()..=*curr.end());
        } else if curr.start() <= prev.end() {
            collapsed.push(*prev.start()..=*prev.end().max(curr.end()));
        } else {
            collapsed.push(prev);
            collapsed.push(curr);
        }
    }
    collapsed
}
fn read_text(contents: &str) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for line in contents.lines() {
        if let Some((lhs, rhs)) = line.split_once(':') {
            let mut sensor = Position::default();
            let mut beacon = Position::default();
            if let Some(lhsxy) = lhs.trim().strip_prefix("Sensor at") {
                if let Some((lhsx, lhsy)) = lhsxy.split_once(',') {
                    sensor.x = lhsx.trim().strip_prefix("x=").unwrap().parse().unwrap();
                    sensor.y = lhsy.trim().strip_prefix("y=").unwrap().parse().unwrap();
                }
            }
            if let Some(rhsxy) = rhs.trim().strip_prefix("closest beacon is at") {
                if let Some((rhsx, rhsy)) = rhsxy.split_once(',') {
                    beacon.x = rhsx.trim().strip_prefix("x=").unwrap().parse().unwrap();
                    beacon.y = rhsy.trim().strip_prefix("y=").unwrap().parse().unwrap();
                }
            }
            pairs.push(Pair {
                sensor: sensor.to_owned(),
                beacon: beacon.to_owned(),
            });
        }
    }
    pairs
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
    fn dist(&self, other: &Position) -> i32 {
        (other.x.abs_diff(self.x) + self.y.abs_diff(other.y)) as i32
    }
}
#[derive(Debug)]
struct Pair {
    sensor: Position,
    beacon: Position,
}
impl Pair {
    fn dist(&self) -> i32 {
        self.sensor.dist(&self.beacon)
    }
    fn row_range(&self, y: i32, min: i32, max: i32) -> Option<RangeInclusive<usize>> {
        let dist = self.dist();
        if (self.sensor.y - dist) <= y && y <= (self.sensor.y + dist) {
            let xdist = dist - self.sensor.y.abs_diff(y) as i32;
            let lb = (self.sensor.x - xdist).max(min) as usize;
            let ub = (self.sensor.x + xdist).min(max) as usize;
            Some(lb..=ub)
        } else {
            None
        }
    }
}
