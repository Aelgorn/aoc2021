use std::{collections::HashSet, time::SystemTime};

use super::read_input;
use bitvec::prelude::*;
use ndarray::{Array2, ArrayView1};

fn process_input() -> Array2<bool> {
    let input = read_input("d3");
    process_input_str(input)
}
fn process_input_str(input: String) -> Array2<bool> {
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
    let input: Array2<bool> = process_input();
    let gamma = calculate_gamma_p2;
    let epsilon = |col: &ArrayView1<bool>, rows_to_skip: &HashSet<usize>| {
        !calculate_gamma_p2(col, rows_to_skip)
    };
    let oxygen_rating = convert(get_rating(gamma, &input)?);
    let co2_rating = convert(get_rating(epsilon, &input)?);
    Ok(oxygen_rating * co2_rating)
}

fn convert(bits: BitVec) -> u32 {
    bits.into_iter()
        .fold(0, |result, bit| (result << 1) ^ u32::from(bit))
}

fn calculate_gamma(input: &Array2<bool>) -> BitVec {
    let stub: HashSet<usize> = HashSet::new();
    input
        .columns()
        .into_iter()
        .map(|col| calculate_gamma_p2(&col, &stub))
        .collect()
}

fn calculate_gamma_p2(column: &ArrayView1<bool>, rows_to_skip: &HashSet<usize>) -> bool {
    let count = column
        .into_iter()
        .enumerate()
        .filter(|(row_idx, _)| !rows_to_skip.contains(row_idx))
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
    criteria: fn(input: &ArrayView1<bool>, rows_to_skip: &HashSet<usize>) -> bool,
    input: &Array2<bool>,
) -> Result<BitVec, ()> {
    let rows: HashSet<usize> =
        HashSet::from_iter(input.rows().into_iter().enumerate().map(|(i, _)| i));
    let mut rows_to_delete: HashSet<usize> = HashSet::new();
    for col in input.columns().into_iter() {
        let criterion = criteria(&col, &rows_to_delete);
        let to_delete: Vec<usize> = col
            .into_iter()
            .enumerate()
            .filter(|(row_idx, &bit)| !rows_to_delete.contains(row_idx) && bit != criterion)
            .map(|(row_idx, _)| row_idx)
            .collect();

        rows_to_delete.extend(to_delete.iter());

        if (rows.len() - rows_to_delete.len()) == 1 {
            let idx = *rows.difference(&rows_to_delete).into_iter().next().unwrap();
            return Ok(input.row(idx).into_iter().collect());
        }
    }
    Err(())
}
