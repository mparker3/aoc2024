use std::{fs::File, io::{Read, Result}, string};

fn main() -> Result<()> {
    let path = "./sample.txt";
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
    for line in char_grid {
        // horizontals

        lines.push(line.clone());
        // reverse horizontals
        lines.push(line.clone().into_iter().rev().collect::<Vec<_>>())
    }

    

    // define horizontals
    // reverse horizontals
    // define verts
    // reverse verts
    // define diags
    // reverse diags
    lines.iter().map(|line| search_line(line, "XMAS")).sum()

}

fn search_line(haystack: &Vec<char>, needle: &str) -> usize {
    haystack.into_iter().collect::<String>().match_indices(needle).collect::<Vec<_>>().len()
}




