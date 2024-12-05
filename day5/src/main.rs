use core::panic;
use std::{collections::HashMap, fs::File, io::{Read, Result}};

#[derive(Debug)]
struct Rule {
    first: i32, 
    last: i32
}

fn main() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (rules_raw, updates_raw) = contents.split_once("\n\n").unwrap();
    let rules_vec = rules_raw.lines().collect::<Vec<&str>>();
    let updates_vec = updates_raw.lines().collect::<Vec<&str>>();
    let rules = rules_vec.iter().map(|rule| {
        let (before, after) = rule.split_once("|").unwrap();
        Rule{first: before.parse::<i32>().unwrap(), last: after.parse::<i32>().unwrap()}
    }).collect::<Vec<Rule>>();
    let updates = updates_vec.iter().map(|update| {
        update.split(",").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    let rule_lookup_table: HashMap<i32, Vec<i32>> = rules
    .iter()
    .fold(HashMap::new(), |mut acc, rule| {
        acc.entry(rule.first)
            .or_insert_with(Vec::new)
            .push(rule.last);
        acc
    });

    // for each update
    let mut incorrects: Vec<Vec<i32>> = vec![];
    for update in updates.iter() {
        if !is_correct(update, &rule_lookup_table).0 {
            incorrects.push(update.clone());
        }
    }
    let mut corrected_updates = Vec::new();

    for incorrect in incorrects {
        corrected_updates.push(corrected(incorrect, &rule_lookup_table)); 
    }


    let mut middles = Vec::new();
    for update in corrected_updates.iter() {
        // get the middle value of the list. hope they're all odd
        if update.len() % 2 == 0 {
            panic!("update list is not odd");
        }
        let middle = update.len() / 2;
        middles.push(update[middle]);
    }
    println!("{}", middles.iter().sum::<i32>());
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}   

fn is_correct(update: &Vec<i32>, rule_lookup_table: &HashMap<i32,Vec<i32>>) -> (bool, usize) {
    for i in 0..update.len() {
        let current = update[i];
        if let Some(forbidden_befores) = rule_lookup_table.get(&current) {
            let before = update.iter().take(i).collect::<Vec<&i32>>();
            // this runtime is going to be really terrible.
            if forbidden_befores.iter().any(|x| before.contains(&x)) {
                return (false, i);
            }
        }
    }
    return (true, usize::MIN);
}
    

fn corrected(incorrect: Vec<i32>, rule_lookup_table: &HashMap<i32,Vec<i32>>) -> Vec<i32>{
    let mut corrected = incorrect.clone();
    let mut i = 0;
    loop {
        let (correct, incorrect_pos) = is_correct(&corrected, &rule_lookup_table); 
        if correct {
            return corrected;
        }
        // find the rule we're breaking
        let before = corrected.clone().into_iter().take(incorrect_pos).collect::<Vec<i32>>();
        let to_correct = corrected.remove(incorrect_pos);
        let rules = rule_lookup_table.get(&to_correct).unwrap();
        for rule in rules {
            if before.contains(&rule) {
                let new_pos = corrected.iter().position(|x|x == rule).unwrap();
                corrected.insert(new_pos, to_correct);
                break;
            }
        }
        
        i += 1;
        if i > 10000 {
            panic!("infinite loop");
        }
    }
}