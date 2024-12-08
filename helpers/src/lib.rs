use std::{fs::File, io::Read};

pub fn get_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn into_grid(grid_str: &str) -> Vec<Vec<char>> {
    grid_str.split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let result = get_input("test_data/data.txt");
        assert_eq!(result, "foo\n\
        bar\n\
        baz\n\
        qux\n");

    }

    #[test]
    #[should_panic]
    fn test_get_invalid_input() {
        let _ = get_input("file/does/not/exist.txt");
    }

    #[test]
    fn test_into_grid() {
        let grid = into_grid("...\n\
        ...\n\
        ...");
        assert_eq!(grid, vec![vec!['.', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', '.']]);
    }

}
