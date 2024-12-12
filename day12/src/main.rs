use std::collections::HashMap;

use helpers::into_grid;
use helpers_macros::timeit;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
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

    pub fn get_neighboring_coords(&self, row: usize, col: usize) -> Vec<Point> {
        let mut coords = Vec::new();
        let row = row as i32;
        let col = col as i32;
        for (row_offset, col_offset) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row + row_offset;
            let new_col = col + col_offset;
            if new_row >= 0 && new_col >= 0 && new_row < self.data.len() as i32 && new_col < self.data[0].len() as i32 {
                coords.push(Point{x: new_row as usize, y: new_col as usize});
            }
        }
        coords
    }

    pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.data.len())
            .flat_map(move |row| {
                (0..self.data[0].len()).map(move |col| (row, col))
            })
    }
}

impl<T: Clone + Default + Copy> From<Vec<Vec<T>>> for Grid<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }
}

#[timeit]
fn main() {
    let grid_str = helpers::get_input("sample1.txt");
    let grid: Grid<_> = into_grid(&grid_str).into(); // dude what is this function signature
    let mut sets: Vec<Vec<Point>> = Vec::new();
    let mut group_pointers: HashMap<Point, &mut Vec<Point>> = HashMap::new();

    for (x, y) in grid.coords() {
        let neighbors = grid.get_neighboring_coords(x, y);
        // check if any of the neighbors have the same value as the current cell
        // if so, add them to the same set
        let mut added = false; // this is bad sharing state
        for neighbor in neighbors {
            if grid.get(neighbor.x, neighbor.y) == grid.get(x, y) { // this is 
                let other_set = group_pointers.get_mut(&neighbor).unwrap();
                // mutate the underlying set and add the current cell to it
                other_set.push(Point{x, y});
                added = true;
                break;
            }   
        }
        if !added {
            TODO(mparker): fix your mutable borrow issues
            let mut new_vec = vec![Point{x, y}];
            sets.push(new_vec);
            let last_vec = sets.last_mut().unwrap();
            group_pointers.insert(Point{x, y}, &mut last_vec);

        }

        // if not, create a new set
    }

}
