use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};
use indicatif::{ProgressBar, ProgressStyle};

use crate::parser::{Block, Spec, spec};
use crate::type_check::{TypeChecker};

use crate::analysis::{Analyser};
use crate::description::{Tag, Description, InforMap};
use crate::lib_spec_processor::{process_lib_specs};
use crate::spec_map::{PropSpecs};
use crate::run_matching::{gen_match_script, run_matching, cleanup_script, setup_dirs};

const CODEGEN: &str = "/*CODEGEN*/\n";
const CODEGENEND: &str = "/*ENDCODEGEN*/\n";

const CODE: &str = "/*CODE*/";
const CODEEND: &str = "/*ENDCODE*/";

const SPEC: &str = "/*SPEC*";
const SPECEND: &str = "*ENDSPEC*/";

const LIB: &str = "./src/library/";
const MATCHSCRIPT: &str = "./racket_specs/gen_match/match-script.rkt";

const LIBMODULE: &str = "preprocess::library::";

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

pub fn process_con_decl(ctx: &InforMap, prop_specs: &PropSpecs) -> Result<String, ErrorMessage> {
    let mut code = String::new();
    for (id, tag) in ctx.iter() {
        match tag {
            Tag::Con(tags) => {
                let descs: Vec<Description> = 
                    tags.iter()
                    .filter(| t | t.is_prop_tag())
                    .map(| t | t.extract_desc())
                    .collect();
                let lookup_result = library_spec_lookup(id.to_string(), descs, prop_specs);
                match lookup_result {
                    Ok(struct_choices) => {
                        if struct_choices.is_empty() {
                            return Err("Unable to find a struct which matches the specification in the library".to_string());
                        } else {
                            // currently we choose the first struct choice
                            code = code + "type " + id + " = " + &struct_choices[0]+ ";\n";
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            _ => continue
        }
    }
    let result = CODEGEN.to_owned() + &code + CODEGENEND;
    Ok(result)
}

fn library_spec_lookup(id: String, descs: Vec<Description>, prop_specs: &PropSpecs) -> Result<Vec<String>, ErrorMessage> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "(>'w')>",
                " ('w') ",
                "^('w')^",
                " ('w') ",
                "<('w'<)",
                " ('w') ",
                "^('w')^",
                " ('w') ",
                "Y('w')Y",
            ])
            .template("{spinner:.magenta} {msg}"),
    );
    pb.set_message("Finding library implementations for ".to_owned() + &id + "...");
    let lib_spec = process_lib_specs(LIB.to_string()).expect("Error: Unable to process library files"); // The specifications of library structs
    let mut structs = Vec::new();
    for (name, lib_file) in lib_spec.iter() {
        let mut is_match = false;
        for desc in &descs {
            let prop_file = prop_specs.get(desc).expect(&("Error: No property specification found for: ".to_string() + &desc));
            match gen_match_script(desc.to_string(), prop_file.to_string(), lib_file.to_string()) {
                Ok(_) => {
                    let result = run_matching(MATCHSCRIPT.to_string());
                    match result {
                        Ok(r) => { // true - match; false - not match
                            if (r) {
                                is_match = true;
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if (is_match) {
            structs.push(LIBMODULE.to_string() + name);
        }
    }
    pb.finish_with_message("Done. ".to_owned() + &structs.len().to_string() + " implementation(s) for " + &id + " found.");
    cleanup_script();
    Ok(structs)
}

pub fn process_src(filename : String) -> Result<String, ErrorMessage> {
    setup_dirs();
    println!("{}", "Ready...");
    let f = readfile(filename);
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
                            // generate con types according to the information in con decl
                            match process_con_decl(analyser.get_ctx(), analyser.get_prop_specs()) {
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