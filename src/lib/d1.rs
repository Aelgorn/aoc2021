use super::read_input;

fn process_input() -> Vec<u32> {
    let input = read_input("d1");
    input.lines().map(|s| s.parse::<u32>().unwrap()).collect()
}

fn depth_increase_sum(input: &[u32]) -> u32 {
    input.windows(2).fold(0u32, |acc, window| {
        let prev = window[0];
        let current = window[1];
        if current > prev {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn run_part1() -> u32 {
    let input = process_input();
    depth_increase_sum(&input)
}

pub fn run_part2() -> u32 {
    let input = process_input();
    let windowed_depths: Vec<u32> = input.windows(3).map(|w| w.iter().sum()).collect();
    depth_increase_sum(&windowed_depths)
}
