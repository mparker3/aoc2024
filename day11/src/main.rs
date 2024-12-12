use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let mut input: Vec<u64> = helpers::get_input("input.txt").split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut cache: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    let blinks = 75;

    println!("{}", input.into_iter().flat_map(|n| rec_blink(n, blinks, &mut cache)).collect::<Vec<u64>>().len());
}

fn rec_blink(i: u64, num_itr: u64, cache: &mut HashMap<(u64, u64), Vec<u64>>) -> Vec<u64> {
    if num_itr == 0 {
        return vec![i]
    }
    if let Some(cached) = cache.get(&(i, num_itr)) {
        println!("Cache hit for ({}, {})", i, num_itr);
        return cached.clone();
    }
    println!("Cache miss for ({}, {})", i, num_itr);

    let mut res: Vec<u64>;
    if i == 0 {
        res = rec_blink(1, num_itr - 1, cache);
    } else if i.to_string().len() % 2 == 0 {
        let str_form = i.to_string();
        let (j, k)= str_form.split_at(str_form.len() / 2);
        res = rec_blink(j.parse::<u64>().unwrap(), num_itr - 1, cache);
        res.extend(rec_blink(k.parse::<u64>().unwrap(), num_itr - 1, cache)); 
    } else {
        res = rec_blink(i* 2024, num_itr- 1, cache)
    }

    cache.insert((i, num_itr), res.clone());
    res
}


fn blink(i: u64) -> Vec<u64>{
    if i == 0 {
        return vec![1];
    } else if i.to_string().len() % 2 == 0 {
        let str_form = i.to_string();
        let (j, k)= str_form.split_at(str_form.len() / 2);
        return vec![j.parse::<u64>().unwrap(), k.parse::<u64>().unwrap()];
    } else {
        return vec![i * 2024];
    }
}