use std::{fs::File, io::Read, io::Result, };
use regex::Regex;

fn main() -> Result<()>{
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"mul\((?P<first>\d*),(?P<second>\d*)\)").unwrap(); 
    let mut muls = vec![];
    for (_, [first, second]) in re.captures_iter(&contents).map(|c| c.extract()) {
        muls.push(first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap());
    }
    println!("{}", muls.iter().sum::<i32>());
    Ok(())
}