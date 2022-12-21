use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut scene = build_scene(contents, 1000, 1000);
    let mut count = 0;

    for _ in 0..1000 {
        let mut cur = Coord::new_sand();
        while cur.y < scene.len() - 1 {
            if scene[cur.y + 1][cur.x] == '.' {
                cur.y += 1;
            } else if scene[cur.y + 1][cur.x - 1] == '.' {
                cur.y += 1;
                cur.x -= 1;
            } else if scene[cur.y + 1][cur.x + 1] == '.' {
                cur.y += 1;
                cur.x += 1;
            } else {
                scene[cur.y][cur.x] = 'o';
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
fn part2(contents: &str) {
    let mut scene = build_scene(contents, 100000, 3);

    let j = scene.len() - 1;
    for i in 0..scene[0].len() {
        scene[j][i] = '#';
    }

    let root = Coord::new_sand();

    let mut count = 0;
    let y_bound = scene.len() - 1;

    for _ in 0..10000000 {
        let mut cur = Coord::new_sand();
        while cur.y < y_bound {
            if scene[cur.y + 1][cur.x] == '.' {
                cur.y += 1;
            } else if scene[cur.y + 1][cur.x - 1] == '.' {
                cur.y += 1;
                cur.x -= 1;
            } else if scene[cur.y + 1][cur.x + 1] == '.' {
                cur.y += 1;
                cur.x += 1;
            } else {
                scene[cur.y][cur.x] = 'o';
                count += 1;
                break;
            }
        }
        if cur.x == root.x && cur.y == root.y {
            break;
        }
    }
    println!("{}", count);
}
fn build_scene(contents: &str, x_offset: usize, y_offset: usize) -> Vec<Vec<char>> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        let mut coords = Vec::new();
        for part in line.split("->") {
            if let Some((x, y)) = part.trim().split_once(',') {
                let coord = Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                };
                coords.push(coord);
            } else {
                panic!();
            }
        }
        lines.push(Line::from_coords(coords));
    }

    let x = lines.iter().map(|l| l.get_max_x()).max().unwrap() + x_offset;
    let y = lines.iter().map(|l| l.get_max_y()).max().unwrap() + y_offset;

    let mut scene = Vec::new();
    for _ in 0..y {
        scene.push(vec!['.'; x]);
    }

    for line in lines.iter() {
        let mut it = line.coords.iter();
        let mut lhs = it.next().unwrap();
        for rhs in it {
            if lhs.x == rhs.x {
                for y in lhs.y.min(rhs.y)..lhs.y.max(rhs.y) + 1 {
                    scene[y][lhs.x] = '#';
                }
            } else {
                for x in lhs.x.min(rhs.x)..lhs.x.max(rhs.x) + 1 {
                    scene[lhs.y][x] = '#';
                }
            }
            lhs = rhs;
        }
    }
    scene
}
#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn new_sand() -> Self {
        Coord { x: 500, y: 0 }
    }
}
struct Line {
    coords: Vec<Coord>,
}
impl Line {
    fn from_coords(coords: Vec<Coord>) -> Self {
        Line { coords }
    }
    fn get_max_x(&self) -> usize {
        self.coords.iter().map(|c| c.x).max().unwrap()
    }
    fn get_max_y(&self) -> usize {
        self.coords.iter().map(|c| c.y).max().unwrap()
    }
}
fn print_scene(scene: &Vec<Vec<char>>) {
    for i in 0..scene.len() {
        for j in 0..scene[0].len() {
            print!("{}", scene[i][j]);
        }
        println!();
    }
}
