use std::env;
use std::fs;

use crate::parser::{Id, Type, spec};

pub fn readfile(filename : String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

#[cfg(test)]
mod tests {
    use crate::generator::{readfile};
    use crate::parser::{spec};

    #[test]
    fn test_read_file() {
        assert!(
            readfile("./spec_code/example.rs".to_string()).len() != 0
        );
    }

    #[test]
    fn test_parse_file() {
        let f = readfile("./spec_code/example.rs".to_string());
        assert!(spec::prog(&f).is_ok())
    }
}