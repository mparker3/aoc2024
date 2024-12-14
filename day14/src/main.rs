use std::collections::HashMap;

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
    print_grid(&robots, dims);
    for i in 0..100 {
        for robot in robots.iter_mut() {
            robot.step(dims.0, dims.1);
        }
        print_grid(&robots, dims);
    }
    println!("{}", partition_grid(&robots, dims).into_iter().map(|x| x.len()).product::<usize>());
}

fn partition_grid(robots: &Vec<Robot>, dims: (isize, isize)) -> Vec<Vec<Robot>> {
    let x_div = dims.0 / 2;
    let y_div = dims.1 / 2;
    let mut grid: Vec<Vec<Robot>> = vec![vec![]; 4];
    for robot in robots {
        if !(robot.position.x == x_div || robot.position.y == y_div) {
            if robot.position.x < x_div {
                if robot.position.y < y_div {
                    grid[0].push(robot.clone());
                } else {
                    grid[1].push(robot.clone());
                }
            } else {
                if robot.position.y < y_div {
                    grid[2].push(robot.clone());
                } else {
                    grid[3].push(robot.clone());
                }
            }
        }
    }
    grid
}

fn print_grid(robots: &Vec<Robot>, dims: (isize, isize)) {
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
}
