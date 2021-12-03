use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

use crate::parser::{Block, Spec, Description, Type, spec};
use crate::type_check::{TypeChecker};
use crate::ctx::{Ctx};
use crate::lib_specs::{construct_spec};

use crate::analysis::{Analyser};
use crate::description::{Tag, InforMap};

const CODEGEN: &str = "/*CODEGEN*/\n";
const CODEGENEND: &str = "/*ENDCODEGEN*/\n";

const CODE: &str = "/*CODE*/";
const CODEEND: &str = "/*ENDCODE*/";

const SPEC: &str = "/*SPEC*";
const SPECEND: &str = "*ENDSPEC*/";

type ErrorMessage = String;

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

pub fn process_block(block: &Block) -> String {
    match block {
        Block::SpecBlock(_, _) => {
            String::new()
        },
        Block::CodeBlock(code, n) => {
            code.to_string()
        }
    }
}

pub fn process_con_decl(ctx: &InforMap) -> Result<String, ErrorMessage> {
    let mut code = String::new();
    for (id, tag) in ctx.iter() {
        match tag {
            Tag::Con(tags) => {
                let descs: Vec<Description> = 
                    tags.iter()
                    .filter(| t | t.is_prop_tag())
                    .map(| t | t.extract_desc())
                    .collect();
                let struct_choices = library_spec_lookup(descs);
                if struct_choices.is_empty() {
                    return Err("Unable to find a struct which matches the specification in the library".to_string());
                } else {
                    // currently we choose the first struct choice
                    code = code + "type " + id + " = " + &struct_choices[0]+ ";\n";
                }
            },
            _ => continue
        }
    }
    let result = CODEGEN.to_owned() + &code + CODEGENEND;
    Ok(result)
}

fn library_spec_lookup(descs: Vec<Description>) -> Vec<String> {
    let lib_spec = construct_spec(); // The specifications of library structs
    let mut structs = Vec::new();
    for (name, _) in lib_spec.iter() {
        if lib_spec.contains_descs(&name, &descs) {
            structs.push(name.to_string());
        }
    }
    structs
}

pub fn process_src(filename : String) -> Result<String, ErrorMessage> {
    let f = readfile("./spec_code/example.rs".to_string());
    match spec::prog(&f) {
        Ok(blocks) => {
            let mut tc = TypeChecker::new();
            match tc.check_prog(blocks.clone()) {// type checking
                Ok(_) => {
                    // type checking ok
                    // run analyser
                    let mut analyser = Analyser::new();
                    match analyser.analyse_prog(blocks.clone()) {
                        Ok(_) => {
                            let mut result = String::new();
                            // generate con types according to the infomation in con decl
                            match process_con_decl(analyser.get_ctx()) {
                                Ok(code) => {
                                    result = result + &code;
                                    // generate rust source code
                                    let code_blocks: Vec<&Block> = 
                                        blocks.iter()
                                        .filter(| block | block.is_code_block())
                                        .collect();
                                    for block in code_blocks.iter() {
                                        result = result + &process_block(block.to_owned());
                                    }
                                    Ok(result)
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(e)
            }
        },
        _ => Err("Error, invalid source code.".to_string())
    }
}

pub fn run(input: String, output_file: String) -> Result<(), Error> {
    match process_src(input) {
        Ok(code) => writefile(output_file, code),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string()))
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
    use crate::generator::{readfile, mark_src_blocks};
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

}