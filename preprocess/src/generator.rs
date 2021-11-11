use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error};

use crate::parser::{Block, Spec, spec};

const CODEGEN: &str = "/*CODEGEN*/\n";
const CODEGENEND: &str = "\n/*CODEGENEND*/\n";

pub fn readfile(filename : String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

pub fn writefile(filename : String, contents: String) -> Result<(), Error> {
    let path = "./gen_code/";

    let mut output = fs::File::create(path.to_owned() + &filename)?;
    write!(output, "{}", contents)?;

    Ok(())
}

pub fn extract_code(block: Block) -> String {
    match block {
        Block::SpecBlock(spec, n) => {
            process_spec(spec.to_vec())
        },
        Block::CodeBlock(code, n) => {
            code.to_string()
        }
    }
}

pub fn process_spec(s: Spec) -> String {
    // TODO : actually process the spec
    let code: &str = "type UniqueCon<T> = std::collections::HashSet<T>;";
    let result = CODEGEN.to_owned() + code + CODEGENEND;
    result
}

pub fn process_src(filename : String) -> String {
    let f = readfile("./spec_code/example.rs".to_string());
    match spec::prog(&f) {
        Ok(blocks) => {
            let mut result = String::new();
            for block in blocks.iter() {
                result = result + &extract_code(block.to_owned());
            }
            result
        },
        _ => "Error, invalid source code.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::{readfile, process_spec};
    use crate::parser::{Decl, spec};

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

    #[test]
    fn test_process_spec() {
        assert!(process_spec(Vec::<Decl>::new()).len() != 0);
    }
}