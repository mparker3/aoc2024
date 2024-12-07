use std::time::Instant;

use helpers::get_input;

struct Equation {
    output: u64,
    inputs: Vec<u64>,
}
impl Equation {
    fn is_solvable(&self) -> bool {
        let [car, cdr @ ..] = self.inputs.as_slice()
        else { panic!("empty input") };

        return can_hit_target(self.output, *car, cdr)
    }
}

fn can_hit_target(target: u64, current: u64, remaining: &[u64]) -> bool {
    match remaining {
        [car, cdr @ ..] => {
            if current > target {
                return false
            }
            can_hit_target(target, current + car, cdr) 
            || can_hit_target(target, current * car, cdr)
            || can_hit_target(target, concat(current, *car), cdr)
        }
        [] => current == target
    }
}

fn concat(head: u64, tail: u64) -> u64 {
    // get tail down to the nearest power of 10
    let mut power: u64 = 1;
    let mut closest = tail;
    while closest >= 10 {
        power *= 10;
        closest /= 10;
    }
    // take to the next place
    power *= 10;
    (head * power) + tail
}



fn main() {
    let now = Instant::now();
    let input = get_input("input.txt").unwrap();
    let eqns = input.split("\n").map(|ln| {
        let (raw_output, raw_inputs) = ln.split_once(": ").unwrap();
        let parsed_inputs = raw_inputs.split(" ").map(|x| {
            x.parse::<u64>().unwrap()
        }).collect(); // TODO(mparker): do i need the early `collect` here?
        Equation{
            output: raw_output.parse::<u64>().unwrap(),
            inputs: parsed_inputs,
        }
    });

    let mut result = 0;
    for eqn in eqns {
        if eqn.is_solvable() {
            result += eqn.output;
        }     
    }
    println!("result: {}", result);
    println!("Took {:?}", now.elapsed());

}
