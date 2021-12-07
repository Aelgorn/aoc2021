use std::{collections::HashMap, hash::Hash, iter};

use derive_more::{Add, Sub};
use regex::Regex;

use crate::read_input;

fn process_input() -> Vec<Line> {
    let input = read_input("d5");
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .filter_map(|l| re.captures(l))
        .map(|captures| Line {
            start: Coords::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            end: Coords::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        })
        .collect()
}

fn run(part1: bool) -> usize {
    let input = process_input();
    let map = input
        .into_iter()
        .flat_map(|line| line.get_all_coords(part1))
        .fold(HashMap::new(), |mut map, coords| {
            *map.entry(coords).or_insert(0) += 1usize;
            map
        });
    map.into_iter().filter(|&(_, count)| count >= 2).count()
}

pub fn run_part1() -> usize {
    run(true)
}
pub fn run_part2() -> usize {
    let input = process_input();
    let map = input
        .into_iter()
        .flat_map(|line| line.get_all_coords(false))
        .fold(HashMap::new(), |mut map, coords| {
            *map.entry(coords).or_insert(0) += 1usize;
            map
        });
    map.into_iter().filter(|&(_, count)| count >= 2).count()
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    start: Coords,
    end: Coords,
}
impl Line {
    pub fn get_all_coords(self, part1: bool) -> Box<dyn Iterator<Item = Coords>> {
        let v = self.end - self.start;
        if v.x == 0 {
            let range = if v.y.is_positive() {
                self.start.y..=self.end.y
            } else {
                self.end.y..=self.start.y
            };
            Box::new(range.map(move |y| Coords::new(self.start.x, y)))
        } else if v.y == 0 {
            let range = if v.x.is_positive() {
                self.start.x..=self.end.x
            } else {
                self.end.x..=self.start.x
            };
            Box::new(range.map(move |x| Coords::new(x, self.start.y)))
        } else if part1 {
            Box::new(iter::empty())
        } else {
            let slope = v.y / v.x;
            let quantity = v.x.abs() + 1;
            Box::new(
                (0..quantity)
                    .map(move |i| {
                        let y = v.y * i / (quantity - 1);
                        let x = y / slope;
                        Coords::new(x, y)
                    })
                    .map(move |c| c + self.start),
            )
        }
    }
}

#[derive(Debug, Copy, Clone, Add, Sub, Hash, PartialEq, Eq)]
pub struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
