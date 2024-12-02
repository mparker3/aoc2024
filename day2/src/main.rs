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
    if is_safe_increasing(report) == -1 || is_safe_decreasing(report) == -1 {
        return true
    }
    // generate all possible permutations of removing an element from the report
    let mut possible_reports = Vec::new();
    for (idx, _) in report.iter().enumerate() {
        let mut report_copy = report.clone();
        report_copy.remove(idx);
        possible_reports.push(report_copy);
    }
    
    for report in possible_reports {
        if is_safe_increasing(&report) == -1 || is_safe_decreasing(&report) == -1 {
            return true
        }
    }
    return false
}
fn is_safe_increasing(report: &Vec<i32>) -> i32 {
    return is_safe(report, true, lt)
}

fn is_safe_decreasing(report: &Vec<i32>) -> i32 {
    return is_safe(report, false, gt)
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

        assert!(check_is_safe(&vec![48, 46, 47, 49, 51, 54, 56]));
        assert!(check_is_safe(&vec![1,1,2,3,4,5]));
        assert!(check_is_safe(&vec![1,2,3,4,5,5]));
        assert!(check_is_safe(&vec![1,4,3,2,1]));
        assert!(check_is_safe(&vec![1,6,7,8,9]));
        assert!(check_is_safe(&vec![1,2,3,4,3]));
        assert!(check_is_safe(&vec![9,8,7,6,7]));
        assert!(check_is_safe(&vec![7,10,8,10,11]));
        assert!(check_is_safe(&vec![29,28,27,25,26,25,22,20]));
        
        assert!(check_is_safe(&vec![5, 4, 3, 2, 1]));
        assert!(check_is_safe(&vec![1, 2, 2, 3, 4]));
        assert!(check_is_safe(&vec![1, 2, 1, 3, 4])); 

        assert!(!check_is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!check_is_safe(&vec![9, 7, 6, 2, 1]));

    }

}
