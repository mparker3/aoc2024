use helpers::get_input;
use helpers_macros::timeit;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Game {
    p: f64,
    q: f64,
    a_x: f64,
    a_y: f64,
    b_x: f64,
    b_y: f64,
}

impl Game {
    fn play(&self) -> Option<f64> {
        let mtrx = Matrix2::new(self.a_x, self.b_x, self.a_y, self.b_y);
        let vec = Vector2::new(self.p, self.q);
        let inv = mtrx.try_inverse()?;
        let sol = inv * vec;
        if (sol[0] - sol[0].round()).abs() > 1e-2 || (sol[1] - sol[1].round()).abs() > 1e-2 {
            return None;
        }
        println!("{:?}", sol);
        return Some(3.0 * sol[0] + 1.0 * sol[1]);
    }
}


#[timeit]
fn main() {
    let input = get_input("input.txt");
    let mut games_raw = input.split("\n\n");

    let re = Regex::new(r"Button A: X\+(?P<a_x>\d+), Y\+(?P<a_y>\d+)\nButton B: X\+(?P<b_x>\d+), Y\+(?P<b_y>\d+)\nPrize: X=(?P<p_x>\d+), Y=(?P<p_y>\d+)").unwrap();
    let mut total_cost: f64 = 0.0;
    for game_raw in games_raw {
        let captures    = re.captures(game_raw).unwrap();
        let game = Game {
            p: captures.name("p_x").unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0,
            q: captures.name("p_y").unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0,
            a_x: captures.name("a_x").unwrap().as_str().parse::<f64>().unwrap(),
            a_y: captures.name("a_y").unwrap().as_str().parse::<f64>().unwrap(),
            b_x: captures.name("b_x").unwrap().as_str().parse::<f64>().unwrap(),
            b_y: captures.name("b_y").unwrap().as_str().parse::<f64>().unwrap(),
        };
        match game.play() {
            Some(cost) => { total_cost += cost; }
            None => {}
        };
    }
    println!("{}", total_cost);
}


