use std::{collections::{HashMap, HashSet}, hash::RandomState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32 
}

fn main() {
    let grid = helpers::into_grid(&helpers::get_input("sample.txt"));

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
    // brute force it. TODO(mparker): use rayon if this is too slow
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let current_loc = Point{x: i as i32, y: j as i32};
            for (letter, antennas) in &points_by_letter {
                let found = is_antinode(&current_loc, &antennas, *letter);
                if found {
                    antinodes.insert(current_loc);
                }
            }
        }
    }
    println!("num_antinodes: {}", antinodes.len())

}

// runs in O(# antennas ^2)
fn is_antinode(point: &Point, antennas: &Vec<Point>, ltr: char) -> bool {
    // create hashmap of antennas -> distances to point
    for antenna in antennas {
        let possible_loc = compute_double_delta(point, antenna);
        for antenna2 in antennas {
            if *antenna2 == possible_loc && antenna != antenna2 && antenna != point {
                println!("found antinode at {:?} for letter {} with antennas {:?} and {:?}", point, ltr, antenna, antenna2);
                return true
            }
        }
    }
    false
}

fn compute_double_delta(p1: &Point, p2: &Point) -> Point {
    return Point{ x: p1.x + ((p2.x - p1.x) * 2), y: p1.y + ((p2.y - p1.y) * 2) }
}
