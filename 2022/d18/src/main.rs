use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let coords = read(contents);
    let value = coords
        .iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| !coords.contains(c))
        .count();
    println!("{:?}", value);
}
fn part2(contents: &str) {
    let coords = read(contents);
    let exposed = exposed(&coords);
    let value = coords
        .iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| exposed.contains(c))
        .count();
    println!("{:?}", value);
}
fn read(contents: &str) -> HashSet<Coord> {
    contents
        .lines()
        .map(|l| {
            let mut n = l.split(',').map(|c| c.parse().unwrap());
            Coord {
                x: n.next().unwrap(),
                y: n.next().unwrap(),
                z: n.next().unwrap(),
            }
        })
        .collect()
}
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}
impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        for dimension in [Dimension::X, Dimension::Y, Dimension::Z] {
            for offset in [-1, 1] {
                let neighbour = match dimension {
                    Dimension::X => Coord {
                        x: self.x + offset,
                        ..*self
                    },
                    Dimension::Y => Coord {
                        y: self.y + offset,
                        ..*self
                    },
                    Dimension::Z => Coord {
                        z: self.z + offset,
                        ..*self
                    },
                };
                neighbours.push(neighbour);
            }
        }
        neighbours
    }
    fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}
enum Dimension {
    X,
    Y,
    Z,
}
fn exposed(coords: &HashSet<Coord>) -> HashSet<Coord> {
    let mut result = HashSet::new();
    let bounds = bounds(coords);

    let start = Coord::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    seen.insert(start);
    stack.push(start);

    while let Some(current) = stack.pop() {
        for neighbour in current.neighbours() {
            if coords.contains(&neighbour) || !neighbour.in_bounds(&bounds) {
                continue;
            }
            if seen.insert(neighbour) {
                stack.push(neighbour);
                result.insert(neighbour);
            }
        }
    }

    result
}
fn bounds(coords: &HashSet<Coord>) -> [Coord; 2] {
    coords.iter().fold(
        [Coord::default(), Coord::default()],
        |[mut mins, mut maxs], coord| {
            mins.x = mins.x.min(coord.x);
            mins.y = mins.x.min(coord.y);
            mins.z = mins.x.min(coord.z);
            maxs.x = maxs.x.max(coord.x);
            maxs.y = maxs.x.max(coord.y);
            maxs.z = maxs.x.max(coord.z);
            [mins, maxs]
        },
    )
}
