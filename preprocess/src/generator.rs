use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error};

use crate::parser::{Block, Spec, spec};

const CODEGEN: &str = "/*CODEGEN*/\n";
const CODEGENEND: &str = "\n/*ENDCODEGEN*/\n";

const CODE: &str = "/*CODE*/";
const CODEEND: &str = "/*ENDCODE*/";

const SPEC: &str = "/*SPEC*";
const SPECEND: &str = "*ENDSPEC*/";

pub fn readfile(filename : String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
        mark_src_blocks(contents)
}

pub fn writefile(filename : String, contents: String) -> Result<(), Error> {
    let path = "./gen_code/";

    let mut output = fs::File::create(path.to_owned() + &filename)?;
    write!(output, "{}", contents)?;

    Ok(())
}

pub fn process_block(block: Block) -> String {
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
    println!("{:?}", s);
    // select all properties
    // let env = s.filter( ... PropertyDecl )

    // select all type declarations
    // let types = s.filter( ... ConTypeDecl )

    // check well formdness
    // check(types, env)

    // let codes: Vec<str> = types.map( |t| -> generate_type(t, env) )

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
                result = result + &process_block(block.to_owned());
            }
            result
        },
        _ => "Error, invalid source code.".to_string()
    }
}

fn mark_src_blocks(src : String) -> String {
    let mut trimed_src = src.trim();
    let mut result = String::new();
    while trimed_src.len() > 0 {
        match trimed_src.find(SPEC) {
            Some(n) => {
                match trimed_src.find(SPECEND)  {
                    Some(m) => {
                        if (n > 0) {
                            let code = &trimed_src[..n];
                            result = result + CODE + &code + CODEEND;
                        }
                        let spec = &trimed_src[n..(m+SPECEND.len())];
                        trimed_src = &trimed_src[(m+SPECEND.len())..].trim();
                        result = result + &spec;
                    },
                    None => {
                        result = result + CODE + trimed_src + CODEEND;
                        break;
                    }
                }
            },
            None => {
                result = result + CODE + trimed_src + CODEEND;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::generator::{readfile, process_spec, mark_src_blocks};
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