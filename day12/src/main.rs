use std::{collections::{HashMap, HashSet}, mem};

use helpers::into_grid;
use helpers_macros::timeit;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

#[derive(Debug)]
struct UnionFind {
    parent: HashMap<(usize, usize), (usize, usize)>,
    rank: HashMap<(usize, usize), usize>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn make_set(&mut self, x: (usize, usize)) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x, x);
            self.rank.insert(x, 0);
        }
    }

    fn find(&mut self, x: &(usize, usize)) -> (usize, usize) {
        let px = self.parent[x];
        if px != *x {
            let new_parent = self.find(&px);
            self.parent.insert(*x, new_parent);
        }
        self.parent[x]
    }

    fn union(&mut self, x: (usize, usize), y: (usize, usize)) {
        let x_root = self.find(&x);
        let y_root = self.find(&y);

        if x_root != y_root {
            let x_rank = self.rank[&x_root];
            let y_rank = self.rank[&y_root];

            if x_rank > y_rank {
                self.parent.insert(y_root, x_root);
            } else if x_rank < y_rank {
                self.parent.insert(x_root, y_root);
            } else {
                self.parent.insert(y_root, x_root);
                self.rank.insert(x_root, x_rank + 1);
            }
        }
    }
}

pub fn find_regions(grid: &Grid<char>) -> Vec<HashSet<(usize, usize)>> {
    let mut uf = UnionFind::new();
    
    for (row, col) in grid.coords() {
        uf.make_set((row, col));
    }

    for (row, col) in grid.coords() {
        let current_value = grid.get(row, col);
        for neighbor in grid.get_neighboring_coords(row, col) {
            if grid.get(neighbor.x, neighbor.y) == current_value {
                uf.union((row, col), (neighbor.x, neighbor.y));
            }
        }
    }

    let mut regions: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for (row, col) in grid.coords() {
        let root = uf.find(&(row, col));
        regions.entry(root)
            .or_insert_with(HashSet::new)
            .insert((row, col));
    }

    regions.into_values().collect()
}

#[timeit]
fn main() {
    let grid_str = helpers::get_input("input.txt");
    let grid: Grid<_> = into_grid(&grid_str).into(); // dude what is this function signature
    let regions = find_regions(&grid);
    let mut total_cost = 0;
    for region in regions {
        let mut region_perim= 0;
        for member in &region {
            for neighbor in grid.get_neighboring_coords(member.0, member.1) {
                if !region.contains(&(neighbor.x, neighbor.y)) {
                    region_perim+= 1;   
                } 
            }
            if member.0 == 0 || member.0 == grid.data.len() - 1 {
                region_perim += 1;
            }
            if member.1 == 0 || member.1 == grid.data[0].len() - 1 {
                region_perim += 1;
            }
        }
        total_cost += region_perim * region.len();
    }
    println!("{}", total_cost);
}