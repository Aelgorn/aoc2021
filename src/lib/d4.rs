use std::collections::HashMap;

use ndarray::Array2;

use super::read_input;

fn process_input() -> Bingo {
    let input = read_input("d4");
    let mut separated = input.split("\n\n");
    let numbers: Vec<u32> = separated
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let boards: Vec<Board> = separated
        .map(|b| {
            b.lines()
                .flat_map(|l| l.split_whitespace().map(|nb| nb.parse().unwrap()))
                .collect::<Vec<u32>>()
        })
        .map(|b| Array2::from_shape_vec((5, 5), b).unwrap())
        .map(Board::new)
        .collect();
    Bingo { numbers, boards }
}

pub fn run_part1() -> u32 {
    let mut bingo = process_input();
    let winner_board = bingo.play()[0];
    let unmarked_sum: u32 = winner_board.1.get_unmarked_numbers().into_iter().sum();
    unmarked_sum * winner_board.0
}
pub fn run_part2() -> u32 {
    let mut bingo = process_input();
    let win_order = bingo.play();
    let last = win_order.last().unwrap();
    let unmarked_sum: u32 = last.1.get_unmarked_numbers().into_iter().sum();
    unmarked_sum * last.0
}

/// ((row.index, col.index), is_marked)
type NumberIndex = (usize, usize);
type MarkedInfo = (NumberIndex, bool);
#[derive(Debug)]
struct Board {
    numbers: Array2<u32>,
    marked: HashMap<u32, MarkedInfo>,
    is_bingo: bool,
}
impl Board {
    pub fn new(numbers: Array2<u32>) -> Self {
        let marked = numbers
            .indexed_iter()
            .map(|(idx, &n)| (n, (idx, false)))
            .collect();
        Self {
            numbers,
            marked,
            is_bingo: false,
        }
    }
    pub fn mark_nb(&mut self, number: u32) -> Option<()> {
        let index = {
            let info = self.marked.get_mut(&number)?;
            info.1 = true;
            info.0
        };
        if self.is_bingo_nb(index) {
            self.is_bingo = true;
        }
        Some(())
    }
    fn is_bingo_nb(&self, index: NumberIndex) -> bool {
        let (row, col) = index;
        let is_row_bingo = self
            .numbers
            .row(row)
            .into_iter()
            .all(|nb| self.marked.get(nb).unwrap().1);
        let is_col_bingo = self
            .numbers
            .column(col)
            .into_iter()
            .all(|nb| self.marked.get(nb).unwrap().1);
        is_row_bingo || is_col_bingo
    }
    pub fn get_unmarked_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .copied()
            .filter(|nb| !self.marked.get(nb).unwrap().1)
            .collect()
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    pub fn play<'a>(&'a mut self) -> Vec<(u32, &'a Board)> {
        self.numbers
            .clone()
            .into_iter()
            .flat_map(|nb| {
                self.play_number(nb)
                    .map(|idx| (nb, idx))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(nb, idx)| (nb, &self.boards[idx]))
            .collect::<Vec<_>>()
    }
    pub fn play_number<'a>(&'a mut self, number: u32) -> impl Iterator<Item = usize> + 'a {
        self.boards
            .iter_mut()
            .enumerate()
            .filter(|board| !board.1.is_bingo)
            .filter_map(move |(board_idx, board)| board.mark_nb(number).map(|_| (board_idx, board)))
            .filter(|(_, board)| board.is_bingo)
            .map(|(board_idx, _)| board_idx)
    }
}
