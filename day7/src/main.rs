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

        return can_hit_target(self.output, *car, cdr.to_vec())
    }
}

fn can_hit_target(target: u64, current: u64, mut remaining: Vec<u64>) -> bool {
    
    if let [car, cdr @ ..] = remaining.as_slice() {
        return can_hit_target(target, current + car, cdr.to_vec()) || can_hit_target(target, current * car, cdr.to_vec()) // TODO(mparker): are these conversions going to kill me?
    }
    return current == target;
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

    // TODO(mparker): multithread this if makes sense in part 2. should be fairly straightforward
    let mut result = 0;
    for eqn in eqns {
        if eqn.is_solvable() {
            result += eqn.output;
        }     
    }
    println!("result: {}", result);
    println!("Took {:?}", now.elapsed());

}
