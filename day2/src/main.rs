use std::{fs::File, io::Read, io::Result};

fn main() -> Result<()>{
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let reports = contents.split("\n").map(|x| x.split_whitespace().map(|ch| ch.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let safe_count = reports.iter().fold(0, |sc, report|{
        if check_is_safe(report) { 
            sc + 1
        } else {
            sc
        }
    });

    println!("{:?}", safe_count);
    Ok(())
}


fn gt(x: i32, y: i32) -> bool {x > y}
fn lt(x: i32, y: i32) -> bool {x < y}

fn check_is_safe(report: &Vec<i32>) -> bool {
    if report[0] == report[1] {
        // bail early. 
        return false
    }
    let is_increasing = report[0] < report[1];
    let cmp: fn(i32, i32) -> bool;
    if is_increasing {
        cmp = lt; 
    } else {
        cmp = gt;
    }


    let  idx = is_safe(report, is_increasing, cmp);
    if idx == -1 {
        return true
    } else {
        let mut report = report.clone();
        report.remove(idx as usize);
        return is_safe(&report, is_increasing, cmp) == -1
    }
}

fn is_safe(report: &Vec<i32>, is_increasing: bool, cmp: fn(i32, i32) -> bool) -> i32 {
    let mut last_val = report[0];
    for (idx, &curr_val) in report.iter().enumerate().skip(1) {
        if !check_bounds(last_val, curr_val, is_increasing) || !cmp(last_val, curr_val) {
            return idx as i32
        }
        last_val = curr_val;
    }
    return -1
}



fn check_bounds(last_val: i32, curr_val: i32, is_increasing: bool) -> bool {
    if is_increasing {
        last_val + 3 >= curr_val        
    } else {
        last_val - 3 <= curr_val
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_monotonic() {
        assert!(check_is_safe(&vec![1, 2, 3, 4, 5]));
        assert!(check_is_safe(&vec![7, 6, 4, 2, 1]));
        assert!(check_is_safe(&vec![1, 3, 6, 7, 9]));
        assert!(check_is_safe(&vec![1, 3, 2, 4, 5]));
        assert!(check_is_safe(&vec![8, 6, 4, 4, 1]));
        
        assert!(check_is_safe(&vec![5, 4, 3, 2, 1]));
        assert!(check_is_safe(&vec![1, 2, 2, 3, 4]));
        assert!(check_is_safe(&vec![1, 2, 1, 3, 4])); 

        assert!(!check_is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!check_is_safe(&vec![9, 7, 6, 2, 1]));

    }

}
