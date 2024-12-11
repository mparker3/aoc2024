fn main() {
    let mut input: Vec<u64> = helpers::get_input("input.txt").split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    for i in 0..75 {
        // TODO(mparker): do i need to do this in place?
        let mut new_input: Vec<u64> = vec![];
        for j in input.into_iter() {
            let mut transformed= blink(j);
            new_input.append(&mut transformed); // not sure why this has to be mutable
        }
        input = new_input;
        // println!("{:?}", input)
    }
    println!("{:?}", input.len());

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