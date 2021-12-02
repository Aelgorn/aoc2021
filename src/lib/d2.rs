use super::read_input;

use derive_more::{Add, AddAssign, Sum};

#[derive(Add, Sum, AddAssign)]
pub struct Position {
    x: i32,
    z: i32,
}

impl Position {
    pub fn default() -> Self {
        Self::new(0, 0)
    }
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

fn process_input() -> Vec<Position> {
    let input = read_input("d2");
    input
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(' ').collect();
            let direction = split[0];
            let amount: i32 = split[1].parse().unwrap();
            match direction {
                "forward" => Position::new(amount, 0),
                "up" => Position::new(0, -amount),
                "down" => Position::new(0, amount),
                _ => panic!(),
            }
        })
        .collect()
}

pub fn run_part1() -> i32 {
    let input = process_input();
    let final_pos: Position = input.into_iter().sum();
    final_pos.x * final_pos.z
}

pub fn run_part2() -> i32 {
    let input = process_input();
    let mut aim = 0;
    let mut position = Position::default();
    for Position { x, z } in input {
        aim += z;
        position += Position::new(x, aim * x);
    }
    position.x * position.z
}
