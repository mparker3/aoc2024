use std::collections::{HashSet, VecDeque};

use helpers::*;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Clone + Default + Copy> Grid<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            data: vec![vec![Default::default(); y_size]; x_size],
        }
    }

    // TODO(mparker): make this not a copy
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    pub fn get_neighboring_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        let row = row as i32;
        let col = col as i32;
        for row_offset in vec![-1, 1] {
            for col_offset in vec![-1, 1] {
                let new_row = row + row_offset;
                let new_col = col + col_offset;
                if new_row >= 0 && new_col >= 0 && new_row < self.data.len() as i32 && new_col < self.data[0].len() as i32 {
                    coords.push((new_row as usize, new_col as usize));
                }
            }
        }
        coords
    }
}

impl Grid<u8> {
    pub fn get_score(&self, row: usize, col: usize) -> u32 {
        let mut score = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((row, col, 0));
        while !queue.is_empty() {
            let (row, col, height) = queue.pop_front().unwrap();
            if visited.contains(&(row, col)) {
                continue;
            }
            visited.insert((row, col));
            let val = self.get(row, col);
            if val == 9 {
                score += 1;
            }
            for (new_row, new_col) in self.get_neighboring_coords(row, col) {
                if visited.contains(&(new_row, new_col)) {
                    continue;
                }
                if self.get(new_row, new_col) == height + 1 {
                    queue.push_back((new_row, new_col, height + 1));
                }
            }
        }
        score
    }
}

fn main() {
    let input = get_input("sample.txt");
    let rows = input.split("\n").collect::<Vec<&str>>();
    let mut grid = Grid::new(rows.len(), rows[0].len());
    let mut starting_points = vec![];
    for (row, row_str) in rows.iter().enumerate() {
        for (col, col_str) in row_str.chars().enumerate() {
            let val = col_str.to_digit(10).unwrap() as u8;
            grid.set(row, col, val);            
            if val == 0 {
                starting_points.push((row, col));
            }
        }
    }

    let score = starting_points.iter().map(|(row, col)| grid.get_score(*row, *col)).sum::<u32>();

    println!("{}", score);
}
