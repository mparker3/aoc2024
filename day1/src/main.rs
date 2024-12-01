use std::{collections::HashMap, fs::File, io::{Error, Read, Result}, num::ParseIntError, string};


fn main() -> Result<()>{
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lefts = contents.split("\n").map(|x| x.split_ascii_whitespace().next().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let rights = contents.split("\n").map(|x| x.split_ascii_whitespace().last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let rights_counts = rights.iter().copied().fold(HashMap::new(), |mut map, val|{
        map.entry(val).and_modify(|frq| *frq+=1).or_insert(1);
        map
    });
    println!("{}", lefts.iter().fold(0, |mut acc, i|{
        acc += rights_counts.get(&i).cloned().unwrap_or(0) * i;
        acc
    }));


    Ok(())
}



