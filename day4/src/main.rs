use std::{fs::File, io::{Read, Result}, string};

fn main() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let pieces = contents.split("\n");
    let char_grid= pieces.map(|piece| {piece.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    println!("{:?}", search(char_grid));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn search(char_grid: Vec<Vec<char>>) -> usize {
    // carve 3x3 chunks of the grid space
    let mut count = 0;
    for i in 0..char_grid.len() - 2 {
        for j in 0..char_grid[i].len() - 2 {
            let chunk = vec![
                vec![char_grid[i][j], char_grid[i][j+1], char_grid[i][j+2]],
                vec![char_grid[i+1][j], char_grid[i+1][j+1], char_grid[i+1][j+2]],
                vec![char_grid[i+2][j], char_grid[i+2][j+1], char_grid[i+2][j+2]]
            ];
            if check_chunk(&chunk) {
                count += 1;
            }
        }
    }
    count
}



fn check_chunk(haystack: &Vec<Vec<char>>) -> bool {
    if haystack[1][1] != 'A' {
        return false;
    }
    let needle = vec!['M', 'A', 'S'];
    let r2l_diag = vec![haystack[0][0], haystack[1][1], haystack[2][2]];
    let l2r_diag = vec![haystack[0][2], haystack[1][1], haystack[2][0]];

    return (r2l_diag == needle || r2l_diag.into_iter().rev().collect::<Vec<char>>() == needle) && (l2r_diag == needle || l2r_diag.into_iter().rev().collect::<Vec<char>>() == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_chunk() {
        let bad_chunk = vec![
            vec!['M', 'A', 'S'],
            vec!['A', 'M', 'A'],
            vec!['S', 'A', 'M']
        ];
        assert_eq!(check_chunk(&bad_chunk), false);
        
        let good_chunk = vec![
            vec!['M', 'A', 'S'],
            vec!['A', 'A', 'A'],
            vec!['M', 'A', 'S']
        ];
        assert_eq!(check_chunk(&good_chunk), true);
    }
}








