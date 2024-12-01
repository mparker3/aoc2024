use std::{fs::File, io::{Read, Error, Result}, num::ParseIntError, string};


fn main() -> Result<()>{
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut left_vals = Vec::<i64>::new();
    let mut right_vals = Vec::<i64>::new();

    let mut lefts = contents.split("\n").map(|x| x.split_ascii_whitespace().next().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut rights = contents.split("\n").map(|x| x.split_ascii_whitespace().last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    lefts.sort();
    rights.sort();
    let mut diffs: i64 = 0;
    for (left, right) in lefts.iter().zip(rights.iter()) {
        diffs += i64::abs(left - right)
    }
    println!("{}", diffs);

    Ok(())
}



