use std::{fs::File, io::{Read, Result}, string};

fn main() -> Result<()> {
    let path = "./input.txt";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let pieces = contents.split("\n");
    let char_grid= pieces.map(|piece| {piece.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    println!("{:?}", search(char_grid));

    Ok(())
}

fn search(char_grid: Vec<Vec<char>>) -> usize {
    let mut lines: Vec<Vec<char>> = vec![];
    for line in char_grid.clone() {
        // horizontals

        lines.push(line.clone());
        // reverse horizontals
        lines.push(line.clone().into_iter().rev().collect::<Vec<_>>())
        
    }

    for i in 0..char_grid.clone()[0].len() {
        let mut vert: Vec<char> = vec![];
        for j in 0..char_grid.len() {
            vert.push(char_grid[j][i]);
        }
        // verticals
        lines.push(vert.clone());
        // reverse verticals
        lines.push(vert.clone().into_iter().rev().collect::<Vec<_>>());
    }

    let diags = get_all_diagonals(&char_grid);
    for diag in diags {
        lines.push(diag.clone());
        // reverse r2l diags
        lines.push(diag.clone().into_iter().rev().collect::<Vec<_>>());
    } 

    // r2l diagonals

    lines.iter().map(|line| search_line(line, "XMAS")).sum()

}

fn get_all_diagonals<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut diagonals = Vec::new();

    // Right diagonals (top-right to bottom-left)
    for c in 0..cols {
        let mut diagonal = Vec::new();
        let mut i = 0;
        let mut j = c;
        while i < rows && j < cols {
            diagonal.push(matrix[i][j].clone());
            i += 1;
            j += 1;
        }
        if diagonal.len() > 1 {
            diagonals.push(diagonal);
        }
    }
    for r in 1..rows {
        let mut diagonal = Vec::new();
        let mut i = r;
        let mut j = 0;
        while i < rows && j < cols {
            diagonal.push(matrix[i][j].clone());
            i += 1;
            j += 1;
        }
        if diagonal.len() > 1 {
            diagonals.push(diagonal);
        }
    }

    // Left diagonals (top-left to bottom-right)
    for c in 0..cols {
        let mut diagonal = Vec::new();
        let mut i = 0;
        let mut j = c;
        while i < rows && j >= 0 {
            diagonal.push(matrix[i][j].clone());
            i += 1;
            if j == 0 { break; }  // Prevent underflow
            j -= 1;
        }
        if diagonal.len() > 1 {
            diagonals.push(diagonal);
        }
    }
    for r in 1..rows {
        let mut diagonal = Vec::new();
        let mut i = r;
        let mut j = cols - 1;
        while i < rows && j >= 0 {
            diagonal.push(matrix[i][j].clone());
            i += 1;
            if j == 0 { break; }  // Prevent underflow
            j -= 1;
        }
        if diagonal.len() > 1 {
            diagonals.push(diagonal);
        }
    }

    diagonals
}

fn search_line(haystack: &Vec<char>, needle: &str) -> usize {
    haystack.into_iter().collect::<String>().match_indices(needle).collect::<Vec<_>>().len()
}




