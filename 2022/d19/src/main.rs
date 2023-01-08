use derive_builder::Builder;
use regex::Regex;
use std::{collections::VecDeque, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    let bluprints = construct_blueprints(&contents);
    println!("{}", part1(&bluprints));
    println!("{}", part2(&bluprints));
}
fn part1(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|bp| bp.index as u32 * simulate(bp, 24))
        .sum()
}
fn part2(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|bp| simulate(bp, 32))
        .product()
}
fn simulate(blueprint: &Blueprint, threshold: u32) -> u32 {
    println!("== Blueprint {} ==", blueprint.index);

    // each robot produces one of its resource. if the numer of robots equals the resources
    // required to build, then we can build the robot every cycle. we can build at most one
    // robot a cycle, so no more of this type are required.
    let mut max_robots = [u32::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.robots.iter().map(|r| r.reqs[i]).max().unwrap();
    }

    let mut geodes = 0;

    let mut queue = VecDeque::new();
    queue.push_back(State {
        resources: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(state) = queue.pop_front() {
        for i in 0..blueprint.robots.len() {
            if state.robots[i] == max_robots[i] {
                continue;
            }

            let robot = &blueprint.robots[i];
            let wait_time = (0..3)
                .map(|m| {
                    if robot.reqs[m] <= state.resources[m] {
                        0
                    } else if state.robots[m] == 0 {
                        threshold + 1
                    } else {
                        (robot.reqs[m] - state.resources[m] + state.robots[m] - 1) / state.robots[m]
                    }
                })
                .max()
                .unwrap();

            let new_elapsed = state.elapsed + wait_time + 1;
            if threshold <= new_elapsed {
                continue;
            }
            let mut new_resources = [0; 4];
            for k in 0..state.robots.len() {
                new_resources[k] =
                    state.resources[k] + state.robots[k] * (wait_time + 1) - robot.reqs[k];
            }
            let mut new_robots = state.robots;
            new_robots[i] += 1;
            queue.push_back(State {
                resources: new_resources,
                robots: new_robots,
                elapsed: new_elapsed,
            });
        }
        // store max geodes we can generate given the robots and time left
        geodes = geodes.max(state.resources[3] + state.robots[3] * (threshold - state.elapsed));
    }
    geodes
}
#[derive(Builder)]
struct Blueprint {
    index: u8,
    robots: Vec<Robot>,
}
#[derive(Builder, Clone)]
struct Robot {
    reqs: [u32; 4],
}
struct State {
    resources: [u32; 4],
    robots: [u32; 4],
    elapsed: u32,
}
fn construct_blueprints(contents: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"^\w+\s(\d+):\s\w+\s(\w+)[\w\s]+(\d+)\s(\w+)\.\s\w+\s(\w+)[\w\s]+(\d+)\s(\w+)\.\s\w+\s(\w+)[\w\s]+(\d+)\s(\w+)\s\w+\s(\d+)\s(\w+)\.\s\w+\s(\w+)[\w\s]+(\d+)\s(\w+)\s\w+\s(\d+)\s(\w+)\.$").unwrap();
    contents
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let rob1 = RobotBuilder::default()
                .reqs([caps.get(3).unwrap().as_str().parse().unwrap(), 0, 0, 0])
                .build()
                .unwrap();
            let rob2 = RobotBuilder::default()
                .reqs([caps.get(6).unwrap().as_str().parse().unwrap(), 0, 0, 0])
                .build()
                .unwrap();
            let rob3 = RobotBuilder::default()
                .reqs([
                    caps.get(9).unwrap().as_str().parse().unwrap(),
                    caps.get(11).unwrap().as_str().parse().unwrap(),
                    0,
                    0,
                ])
                .build()
                .unwrap();

            let rob4 = RobotBuilder::default()
                .reqs([
                    caps.get(14).unwrap().as_str().parse().unwrap(),
                    0,
                    caps.get(16).unwrap().as_str().parse().unwrap(),
                    0,
                ])
                .build()
                .unwrap();

            BlueprintBuilder::default()
                .index(caps.get(1).unwrap().as_str().parse().unwrap())
                .robots(vec![rob1, rob2, rob3, rob4])
                .build()
                .unwrap()
        })
        .collect()
}
