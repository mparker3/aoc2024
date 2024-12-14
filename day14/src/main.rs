use core::f64;
use std::{collections::HashMap, thread::sleep, time::Duration};

use helpers::get_input;
use helpers_macros::timeit;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Point,
    direction: Point,
}

impl Robot {
    fn step(&mut self, x_max: isize, y_max: isize) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
        if self.position.x < 0 {
            self.position.x += x_max;
        } else if self.position.x >= x_max {
            self.position.x -= x_max;
        }
        if self.position.y < 0 {
            self.position.y += y_max;
        } else if self.position.y >= y_max {
            self.position.y -= y_max;
        }
    }
}

#[timeit]
fn main() {
    let dims: (isize, isize) = (101, 103);
    let input = get_input("input.txt");
    let mut robots: Vec<Robot> = Vec::new();
    // format: p=0,4 v=3,-3
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let (p_raw, v_raw) = (parts[0], parts[1]);
        let p = p_raw.split("=").nth(1).unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let v = v_raw.split("=").nth(1).unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let robot = Robot {
            position: Point { x: p[0] as isize, y: p[1] as isize },
            direction: Point { x: v[0] as isize, y: v[1] as isize },
        };
        robots.push(robot);
    }
    print_grid(&robots, dims, 0);
    let mut min_centrality: f64 = f64::MAX;
    for i in 0..10000 {
        for robot in robots.iter_mut() {
            robot.step(dims.0, dims.1);
        }
        let centrality = get_total_distance(&robots, dims);
        if centrality < min_centrality {
            min_centrality = centrality;
            println!("{}", i);
        print_grid(&robots, dims, i);
        }
    }
}

fn get_total_distance(robots: &Vec<Robot>, dims: (isize, isize)) -> f64 {
    let x_ctr = dims.0 / 2;
    let y_ctr = dims.1 / 2;
    let mut total_dist = 0.0;
    for robot in robots {
        total_dist += ((robot.position.x - x_ctr) as f64).powi(2) + ((robot.position.y - y_ctr) as f64).powi(2);
    }
    total_dist
}

fn print_grid(robots: &Vec<Robot>, dims: (isize, isize), itr: usize) {
    // print!("{}[2J", 27 as char);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; dims.0 as usize]; dims.1 as usize];
    let mut pos_cts: HashMap<Point, usize> = HashMap::new();
    for robot in robots {
        *pos_cts.entry(robot.position).or_insert(0) += 1;
    }
    for (pos, cts) in pos_cts {
        grid[pos.y as usize][pos.x as usize] = cts.to_string().chars().next().unwrap();
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("\n");
    println!("{}", itr);
}
