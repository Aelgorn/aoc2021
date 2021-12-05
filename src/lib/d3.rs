use std::{collections::HashSet, sync::Arc, thread};

use super::read_input;
use bitvec::prelude::*;
use ndarray::{Array2, ArrayView1};
use rayon::prelude::*;

fn process_input() -> Array2<bool> {
    let input = read_input("d3");
    let lines: Vec<&str> = input.lines().collect();
    let first = lines[0];
    let n_cols = first.len();
    let n_rows = lines.len();
    let shape = (n_rows, n_cols);
    let flat: Vec<bool> = lines
        .into_iter()
        .flat_map(|line| line.chars().map(|c| if c == '0' { false } else { true }))
        .collect();
    Array2::from_shape_vec(shape, flat).unwrap()
}

pub fn run_part1() -> u32 {
    let input: Array2<bool> = process_input();
    let gamma: BitVec = calculate_gamma(&input);
    let epsilon: BitVec = !gamma.clone();
    convert(gamma) * convert(epsilon)
}

pub fn run_part2() -> Result<u32, ()> {
    let gamma = calculate_gamma_p2;
    let epsilon = |col: &ArrayView1<bool>| !calculate_gamma_p2(col);
    let oxygen_input: Arc<Array2<bool>> = Arc::new(process_input());
    let co2_input = oxygen_input.clone();
    let oxygen_rating = thread::spawn(move || convert(get_rating(gamma, &*oxygen_input).unwrap()));
    let co2_rating = thread::spawn(move || convert(get_rating(epsilon, &*co2_input).unwrap()));
    let oxygen_rating = oxygen_rating.join().unwrap();
    let co2_rating = co2_rating.join().unwrap();
    Ok(oxygen_rating * co2_rating)
}

fn convert(bits: BitVec) -> u32 {
    bits.into_iter()
        .fold(0, |result, bit| (result << 1) ^ u32::from(bit))
}

fn calculate_gamma(input: &Array2<bool>) -> BitVec {
    input
        .columns()
        .into_iter()
        .map(|col| calculate_gamma_p2(&col))
        .collect()
}

fn calculate_gamma_p2(column: &ArrayView1<bool>) -> bool {
    let count = column
        .into_iter()
        .enumerate()
        .fold((0, 0), |mut count, (_, &bit)| {
            if bit {
                count.1 += 1;
            } else {
                count.0 += 1;
            }
            count
        });
    count.1 >= count.0
}

fn get_rating(
    criteria: fn(input: &ArrayView1<bool>) -> bool,
    input: &Array2<bool>,
) -> Result<BitVec, ()> {
    let mut rows: HashSet<usize> = HashSet::from_iter(0..input.rows().into_iter().len());
    let mut vec: Vec<(usize, HashSet<usize>)> = input
        .columns()
        .into_iter()
        .enumerate()
        .par_bridge()
        .map(|(idx, col)| {
            let criterion = criteria(&col);
            (
                idx,
                col.into_iter()
                    .enumerate()
                    .filter(|(_, &bit)| bit != criterion)
                    .map(|(row_idx, _)| row_idx)
                    .collect(),
            )
        })
        .collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    let mut i = 0;
    while rows.len() > 1 {
        rows = rows.difference(&vec[i].1).copied().collect();
        i += 1;
    }

    if rows.is_empty() {
        return Err(());
    }

    let idx = rows.into_iter().next().unwrap();
    Ok(input.row(idx).into_iter().collect())
}
