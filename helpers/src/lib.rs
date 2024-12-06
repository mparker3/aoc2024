use std::{fs::File, io::{Error, Read}};

pub fn get_input(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let result = get_input("test_data/data.txt").unwrap();
        assert_eq!(result, "foo\n\
        bar\n\
        baz\n\
        qux\n");

        let result = get_input("file/does/not/exist.txt");
        assert!(result.is_err()); 
    }


}
