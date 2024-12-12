use std::{collections::HashMap, sync::{Arc, RwLock}};
use rayon::prelude::*;
use helpers_macros::timeit;

#[timeit]
fn main() {
    let input: Vec<u128> = helpers::get_input("input.txt").split_whitespace().map(|x| x.parse::<u128>().unwrap()).collect();
    let cache: Arc<RwLock<HashMap<(u128, u128), u128>>> = Arc::new(RwLock::new(HashMap::new()));

    let blinks = 208;

    println!("{}", input.into_par_iter().map(|n| {
        let cache_ref = Arc::clone(&cache);
        rec_blink(n, blinks, &cache_ref)}
    ).sum::<u128>());
}

fn rec_blink(i: u128, num_itr: u128, cache_ref: &Arc<RwLock<HashMap<(u128, u128), u128>>>) -> u128 {
    { // read block
        let cache = cache_ref.read().unwrap();
        if let Some(cached) = cache.get(&(i, num_itr)) {
            return cached.clone();
        }
    } // end read block

    if num_itr == 0 {
        return 1;
    }
   
    let res: u128;
    if i == 0 {
        res = rec_blink(1, num_itr - 1, cache_ref);
    } else if i.to_string().len() % 2 == 0 {
        let str_form = i.to_string();
        let (j, k)= str_form.split_at(str_form.len() / 2);
        res = rec_blink(j.parse::<u128>().unwrap(), num_itr - 1, cache_ref) + rec_blink(k.parse::<u128>().unwrap(), num_itr - 1, cache_ref); 
    } else {
        res = rec_blink(i* 2024, num_itr - 1, cache_ref)
    }
    { // write block
        let mut cache = cache_ref.write().unwrap();
        cache.insert((i, num_itr), res.clone());
        res
    } // end write block
}