use std::{collections::HashMap, fs::File, io::{Read, Result}, string};

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
    let mut valid_updates: Vec<Vec<i32>> = vec![];
    for update in updates.iter() {
        let mut valid = true;
        for i in 0..update.len() {
            let current = update[i];
            if let Some(forbidden_befores) = rule_lookup_table.get(&current) {
                let before = update.iter().take(i).collect::<Vec<&i32>>();
                // this runtime is going to be really terrible.
                if forbidden_befores.iter().any(|x| before.contains(&x)) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            valid_updates.push(update.clone());
        }
    }

    let mut middles = Vec::new();
    for update in valid_updates.iter() {
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
