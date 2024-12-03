use std::{fs::File, io::Read, io::Result, };
use regex::Regex;

fn main() -> Result<()>{
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mult_re = Regex::new(r"mul\((?P<first>\d*),(?P<second>\d*)\)").unwrap(); 
    let split_re = Regex::new(r"(?s)(don't\(\)).*?(do\(\))").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let parts = split_re.split(&contents).collect::<Vec<_>>();
    let mut muls = vec![];
    for part in parts.iter().take(parts.len() - 1) {
        println!("{:?}",part);
        for cap in mult_re.captures_iter(part) {
            let first = cap["first"].parse::<i32>().unwrap();
            let second = cap["second"].parse::<i32>().unwrap();
            muls.push(first * second);
        }
    }

    let last_cap = dont_re.split(parts.last().unwrap()).take(1).last();
    let last_muls = mult_re.captures_iter(last_cap.unwrap());
    for cap in last_muls {
        let first = cap["first"].parse::<i32>().unwrap();
        let second = cap["second"].parse::<i32>().unwrap();
        muls.push(first * second);
    }
    

    println!("{}", muls.iter().sum::<i32>());
    Ok(())
}