use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    simulate(&contents, 2);
    simulate(&contents, 10);
}
fn simulate(contents: &str, knots: usize) {
    let mut positions = Vec::new();
    for _ in 0..knots {
        positions.push(Position::default());
    }
    let mut visited = HashSet::new();
    visited.insert(Position::default());

    for line in contents.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let dir = Position::from_char(dir.chars().next().unwrap());
        let steps: i32 = count.parse().unwrap();

        for _ in 0..steps {
            let mut iter = positions.iter_mut();
            let mut prev = iter.next().unwrap();
            prev.move_by(&dir);

            for curr in iter {
                if curr.distance(prev) > 1 {
                    curr.move_by(&curr.delta(prev));
                }
                prev = curr;
            }
            visited.insert(positions.last().unwrap().clone());
        }
    }
    println!("{:?}", visited.len());
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
    fn default() -> Self {
        Position::new(0, 0)
    }
    fn from_char(c: char) -> Self {
        match c {
            'U' => Position::new(0, 1),
            'R' => Position::new(1, 0),
            'D' => Position::new(0, -1),
            'L' => Position::new(-1, 0),
            _ => unreachable!(),
        }
    }
    fn move_by(&mut self, position: &Position) {
        self.x += position.x;
        self.y += position.y;
    }
    fn distance(&self, other: &Position) -> i32 {
        max((other.x - self.x).abs(), (other.y - self.y).abs())
    }
    fn delta(&self, other: &Position) -> Position {
        let mut delta_x = other.x - self.x;
        let mut delta_y = other.y - self.y;
        if delta_x.abs() <= 2 && delta_y.abs() <= 2 {
            delta_x = delta_x.clamp(-1, 1);
            delta_y = delta_y.clamp(-1, 1);
        } else if delta_x.abs() == 2 && delta_y == 0 {
            delta_x = delta_x.clamp(-1, 1);
        } else if delta_x == 0 && delta_y.abs() == 2 {
            delta_y = delta_y.clamp(-1, 1);
        }
        Position::new(delta_x, delta_y)
    }
}
