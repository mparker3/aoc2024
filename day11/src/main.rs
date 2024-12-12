use std::{collections::HashMap, sync::{Arc, RwLock}};
use rayon::prelude::*;
use helpers_macros::timeit;

#[timeit]
fn main() {
    let mut input: Vec<u64> = helpers::get_input("input.txt").split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let cache: Arc<RwLock<HashMap<(u64, u64), u64>>> = Arc::new(RwLock::new(HashMap::new()));

    let blinks = 75;

    println!("{}", input.into_par_iter().map(|n| {
        let cache_ref = Arc::clone(&cache);
        rec_blink(n, blinks, &cache_ref)}
    ).sum::<u64>());
}

fn rec_blink(i: u64, num_itr: u64, cache_ref: &Arc<RwLock<HashMap<(u64, u64), u64>>>) -> u64 {
    { // read block
        let cache = cache_ref.read().unwrap();
        if let Some(cached) = cache.get(&(i, num_itr)) {
            return cached.clone();
        }
    } // end read block

    if num_itr == 0 {
        return 1;
    }
   
    let res: u64;
    if i == 0 {
        res = rec_blink(1, num_itr - 1, cache_ref);
    } else if i.to_string().len() % 2 == 0 {
        let str_form = i.to_string();
        let (j, k)= str_form.split_at(str_form.len() / 2);
        res = rec_blink(j.parse::<u64>().unwrap(), num_itr - 1, cache_ref) + rec_blink(k.parse::<u64>().unwrap(), num_itr - 1, cache_ref); 
    } else {
        res = rec_blink(i* 2024, num_itr - 1, cache_ref)
    }
    { // write block
        let mut cache = cache_ref.write().unwrap();
        cache.insert((i, num_itr), res.clone());
        res
    } // end write block
}