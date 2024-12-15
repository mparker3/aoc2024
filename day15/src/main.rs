use std::collections::{HashMap, HashSet};

use helpers::{get_input, into_grid};
use helpers_macros::timeit;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl<T: Clone + Default + Copy> Grid<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            data: vec![vec![Default::default(); y_size]; x_size],
        }
    }

    pub fn get(&self, row: isize, col: isize) -> T {
        self.data[row as usize][col as usize]
    }

    pub fn set(&mut self, row: isize, col: isize, value: T) {
        self.data[row as usize][col as usize] = value;
    }

    pub fn get_neighboring_coords(&self, row: isize, col: isize) -> Vec<Point> {
        let mut coords = Vec::new();
        for (row_offset, col_offset) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row + row_offset;
            let new_col = col + col_offset;
            if new_row >= 0 && new_col >= 0 && new_row < self.data.len() as isize && new_col < self.data[0].len() as isize {
                coords.push(Point{x: new_row, y: new_col});
            }
        }
        coords
    }

    pub fn coords(&self) -> impl Iterator<Item = (Point, T)> + '_ {
        (0..self.data.len())
            .flat_map(move |row| {
                (0..self.data[0].len()).map(move |col| (Point{x: row as isize, y: col as isize}, self.data[row][col]))
            })
    }
    fn from(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }
}

const DIR_ARRAY: [(char, (isize, isize)); 4] = [
    ('^', (-1, 0)),
    ('<', (0, -1)),
    ('>', (0, 1)),
    ('v', (1, 0)),
];

#[timeit]
fn main() {
    let input = get_input("input.txt");
    let (grid_raw, dirs_raw) = input.split_once("\n\n").unwrap();
    let dirs = dirs_raw.replace("\n", "");
    let mut grid = Grid::from(into_grid(grid_raw));
    let dir_map: HashMap<char, (isize, isize)> = HashMap::from(DIR_ARRAY);
    let mut robot_pos: Point = Point{x: 0, y: 0};
    let mut crate_positions: HashSet<Point> = HashSet::new();
    let mut wall_positions: HashSet<Point> = HashSet::new();
    for entry in grid.coords() {
        let (point, value) = entry;
        match value {
            '@' => robot_pos = point,
            'O' => { crate_positions.insert(point); }
            '#' => { wall_positions.insert(point); }
            _ => {}
        }
    }

    for mv in dirs.chars() {
        let dir = dir_map.get(&mv);
        match dir {
            Some(dir) => robot_pos = try_move(&mut grid, robot_pos, dir),
            None => panic!("Invalid direction: {}", mv),
        }
    }

    let total = grid.coords().map(|(pt, chr)| {
        if chr == 'O' {
            (pt.x * 100) + pt.y
        } else {
            0
        }
    }).sum::<isize>();
    println!("{}", total);
}

fn try_move(grid: &mut Grid<char>, robot_pos: Point, dir: &(isize, isize)) -> Point {
    let affected = assemble_affected_crates(grid, Point{x: robot_pos.x + dir.0, y: robot_pos.y + dir.1}, *dir);
    if affected.is_none() {
        return robot_pos;
    }
    let affected = affected.unwrap();
    if affected.len() > 0 {
        let last_crate = affected.iter().next().unwrap();
        grid.set(last_crate.x + dir.0, last_crate.y + dir.1, 'O');
    }
    grid.set(robot_pos.x, robot_pos.y, '.');
    grid.set(robot_pos.x + dir.0, robot_pos.y + dir.1, '@');
    
    return Point{x: robot_pos.x + dir.0, y: robot_pos.y + dir.1};
}

fn assemble_affected_crates(grid: &Grid<char>, curr_pos: Point, dir: (isize, isize)) -> Option<Vec<Point>> {
    match grid.get(curr_pos.x, curr_pos.y) {
        '#' => return None,
        '.' => return Some(Vec::new()),
        'O' => {
            let next_pos = Point{x: curr_pos.x + dir.0, y: curr_pos.y + dir.1};
            if let Some(mut affected_crates) = assemble_affected_crates(grid, next_pos, dir) {
                affected_crates.push(curr_pos);
                return Some(affected_crates);
            } else {
                return None;
            }
        }
        _ => {
            panic!("Invalid grid value: {}", grid.get(curr_pos.x, curr_pos.y));
        }
    }
}