use helpers::get_input;
use helpers_macros::timeit;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Game {
    target: Point,
    button_a: Point,
    button_b: Point,
}

impl Game {
    fn play(&self) -> Option<u64> {
        let mut costs: Vec<_> = Vec::new();
        for i in 0..100 {
            for j in 0..100 {
                if (i * self.button_a.x) + (j * self.button_b.x) == self.target.x && (i * self.button_a.y) + (j * self.button_b.y) == self.target.y {
                    costs.push(3*i + 1*j);
                }
            }
        }
        if costs.is_empty() {
            return None;
        }
        costs.sort();
        return Some(costs[0]);
    }
}


#[timeit]
fn main() {
    let input = get_input("input.txt");
    let mut games_raw = input.split("\n\n");

    let re = Regex::new(r"Button A: X\+(?P<a_x>\d+), Y\+(?P<a_y>\d+)\nButton B: X\+(?P<b_x>\d+), Y\+(?P<b_y>\d+)\nPrize: X=(?P<p_x>\d+), Y=(?P<p_y>\d+)").unwrap();
    let mut total_cost = 0;
    for game_raw in games_raw {
        let captures    = re.captures(game_raw).unwrap();
        let game = Game {
            button_a: Point {
                x: captures.name("a_x").unwrap().as_str().parse().unwrap(),
                y: captures.name("a_y").unwrap().as_str().parse().unwrap(),
            },
            button_b: Point {
                x: captures.name("b_x").unwrap().as_str().parse().unwrap(),
                y: captures.name("b_y").unwrap().as_str().parse().unwrap(),
            },
            target: Point {
                x: captures.name("p_x").unwrap().as_str().parse().unwrap(),
                y: captures.name("p_y").unwrap().as_str().parse().unwrap(),
            },
        };
        match game.play() {
            Some(cost) => { total_cost += cost; }
            None => {}
        };
    }
    println!("{}", total_cost);
}


