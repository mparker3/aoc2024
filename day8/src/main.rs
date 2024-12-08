use std::{collections::{HashMap, HashSet}, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32 
}

fn main() {
    let now = Instant::now();
    let grid = helpers::into_grid(&helpers::get_input("input.txt"));

    // let mut points: Vec<Point>; 
    let mut points_by_letter = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                points_by_letter.entry(grid[i][j])
                .and_modify(|x: &mut Vec<Point>| x.push(Point{x: i as i32, y: j as i32}))
                .or_insert(vec![Point{x: i as i32, y: j as i32}]);
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, points) in points_by_letter {
        let letter_antinodes = get_antinodes(&points, (grid[0].len() - 1) as i32, (grid.len() - 1) as i32);
        antinodes.extend(letter_antinodes);
    }
    println!("num_antinodes: {}", antinodes.len());
    println!("elapsed: {:?}", now.elapsed())

}

// runs in O(# antennas ^2)
fn get_antinodes(antennas: &Vec<Point>, x_max: i32, y_max: i32) -> Vec<Point> {
    // create hashmap of antennas -> distances to point
    let mut locs: Vec<Point> = vec![];
    for (a1, a2) in get_combinations(antennas) {
        locs.extend(compute_all_deltas(&a1, &a2, x_max, y_max));
    }
    locs
}

fn get_combinations<T>(elems: &Vec<T>) -> Vec<(&T, &T)> {
    let mut combinations = Vec::new();
    for i in 0..elems.len() {
        for j in (i + 1)..elems.len() {
            combinations.push((&elems[i], &elems[j]));  
        }
    }
    combinations
}

fn compute_all_deltas(p1: &Point, p2: &Point, x_max: i32, y_max: i32) -> Vec<Point> {
    let delta = (p2.x - p1.x, p2.y - p1.y);
    let negative_delta = (delta.0 * -1, delta.1 * -1);
    let mut points = Vec::new();
    // forward pass
    let mut current_point = p2.clone();
    while current_point.x <= x_max && current_point.y <= y_max && current_point.x >= 0 && current_point.y >= 0 {
        points.push(current_point);
        current_point.x += delta.0;
        current_point.y += delta.1;
    }

    let mut current_point = p1.clone();
    while current_point.x <= x_max && current_point.y <= y_max && current_point.x >= 0 && current_point.y >= 0 {
        points.push(current_point);
        current_point.x += negative_delta.0;
        current_point.y += negative_delta.1;
    }
    points
}
