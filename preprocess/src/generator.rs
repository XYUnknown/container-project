use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};
use indicatif::{ProgressBar, ProgressStyle};

use crate::parser::{Block, Spec, spec};
use crate::type_check::{TypeChecker};

use crate::analysis::{Analyser};
use crate::description::{Tag, Description, InforMap};
use crate::lib_spec_processor::{process_lib_specs};
use crate::spec_map::{PropSpecs, MatchSetup, ProvidedOps};
use crate::run_matching::{LANGDECL, initialise_match_setup, gen_match_script, run_matching, cleanup_script, setup_dirs};

const CODEGEN: &str = "/*CODEGEN*/\n";
const CODEGENEND: &str = "/*ENDCODEGEN*/\n";

const CODE: &str = "/*CODE*/";
const CODEEND: &str = "/*ENDCODE*/";

const SPEC: &str = "/*SPEC*";
const SPECEND: &str = "*ENDSPEC*/";

const LIB: &str = "./src/library/";
const MATCHSCRIPT: &str = "./racket_specs/gen_match/match-script.rkt";

const IMPORT: &str = "use preprocess::traits::container_constructor::ContainerConstructor;\n";
const TRAITCRATE: &str = "preprocess::traits::";

const OPS: &str = "./racket_specs/gen_lib_spec/ops.rkt";

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

fn process_bound_elem_ty(t: &str, elem_ty: &str) -> String {
    return TRAITCRATE.to_string() + t + "<" + elem_ty + ">";
}

pub fn process_bound_decl(ctx: &InforMap) -> Result<String, ErrorMessage> {
    let mut code = String::new();
    let match_setup = initialise_match_setup();
    for (id, tag) in ctx.iter() {
        match tag {
            Tag::Bound((c, t), decs) => {
                let traits = decs.iter().map(|name| process_bound_elem_ty(name, t)).collect::<Vec<String>>().join(" + ");
                code = code + &gen_trait_code(id, c, t, &traits);
            },
            _ => continue
        }
    }
    Ok(code)
}

pub fn process_con_decl(ctx: &InforMap, prop_specs: &PropSpecs) -> Result<String, ErrorMessage> {
    let mut code = String::new();
    let match_setup = initialise_match_setup();
    for (id, tag) in ctx.iter() {
        match tag {
            Tag::Con(elem_ty, i_name, tags) => {
                let prop_descs: Vec<Description> = 
                    tags.iter()
                    .filter(| t | t.is_prop_tag())
                    .map(| t | t.extract_prop_desc())
                    .collect();
                let bounds: Vec<Description> =
                    tags.iter()
                    .filter(| t | t.is_bound_tag())
                    .flat_map(| t | t.extract_bound_descs())
                    .collect();
                let lookup_result = library_spec_lookup(id.to_string(), prop_descs, bounds, prop_specs, &match_setup);
                match lookup_result {
                    Ok(struct_choices) => {
                        if struct_choices.is_empty() {
                            return Err("Unable to find a struct which matches the specification in the library".to_string());
                        } else {
                            let opt = struct_choices.join(", ");
                            code = code + &gen_output_code(id, elem_ty, &struct_choices[0], i_name, &opt)
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
    Ok(code)
}

fn write_provided_ops(provided_ops: &ProvidedOps) -> Result<(), Error>  {
    let ops_path = OPS;
    let (code, ops) = provided_ops;
    let mut output = fs::File::create(ops_path.to_owned())?;
    write!(output, "{}", LANGDECL.to_string())?;
    for i in 0..code.len() {
        write!(output, "{}", code[i])?;
    }
    let ops_string = ops.join(" ");
    let provide = "\n(provide ".to_string() + &ops_string + ")";
    write!(output, "{}", provide)?;
    Ok(())
}

fn library_spec_lookup(id: String, properties: Vec<Description>, bounds: Vec<Description>, prop_specs: &PropSpecs, match_setup: &MatchSetup) -> Result<Vec<String>, ErrorMessage> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "(>'-')>",
                " ('-') ",
                "<('-'<)",
                " ('-') ",
                "^('-')^",
                " ('-') ",
                "v('-')v",
                " ('-') ",
                " (^-^) ",
            ])
            .template("{spinner:.magenta} {msg}"),
    );
    pb.set_message("Finding library implementations for ".to_owned() + &id + "...");
    let lib_spec = process_lib_specs(LIB.to_string()).expect("Error: Unable to process library files"); // The specifications of library structs
    let mut structs = Vec::new();
    // select library structs implement bounds decl in contype
    let mut lib_spec_impls = lib_spec.clone();
    for (name, (_, impls, _)) in lib_spec.iter() {
        if (!bounds.iter().all(|i| impls.keys().cloned().collect::<String>().contains(i))) {
            lib_spec_impls.remove(name);
        }
    }
    for (name, (lib_spec_dir, bound_ctx, provided_ops)) in lib_spec_impls.iter() {
        match write_provided_ops(provided_ops) {
            Ok(_) => { },
            Err(_) => {
                return Err("Error, cannot obtain provided operations from the library specifiction".to_string());
            }
        }
        let mut is_match = false;
        for p in &properties {
            let mut is_partial_match = false;
            for i in &bounds {
                let (prop_file, symbolics) = prop_specs.get(p).expect(&("Error: No property specification found for: ".to_string() + &p));
                match gen_match_script(p.to_string(), match_setup.get(i).unwrap().to_string(), prop_file.to_string(), lib_spec_dir.to_string(), bound_ctx.get(i).unwrap().to_string(), symbolics) {
                    Ok(_) => {
                        let result = run_matching(MATCHSCRIPT.to_string());
                        match result {
                            Ok(r) => { // true - match; false - not match
                                if (r) {
                                    is_partial_match = true;
                                } else {
                                    is_partial_match = false;
                                    break;
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
            is_match = is_partial_match;
            if (!is_match) {
                break;
            }
        }
        if (is_match) {
            structs.push(name.to_string());
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
                            match process_bound_decl(analyser.get_ctx()) {
                                Ok(code) => {
                                    result = result + &code;
                                    match process_con_decl(analyser.get_ctx(), analyser.get_prop_specs()) {
                                        Ok(code) => {
                                            result = CODEGEN.to_string() + IMPORT + &result + &code + CODEGENEND;
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

pub fn gen_output_code(s: &str, elem_type: &str, chosen: &str, trait_name: &str, choices: &str) -> String {
    format!(
r#"struct {s}<{elem_type}> {{
    elem_t: core::marker::PhantomData<{elem_type}>,
}}

impl<{elem_type}: 'static + Ord + std::hash::Hash> ContainerConstructor for {s}<{elem_type}> {{
    type Impl = {chosen}<{elem_type}>; // All possible choices: {choices}
    type Bound = dyn {trait_name}<{elem_type}>;
    fn new() -> Box<Self::Bound> {{
        Box::new(Self::Impl::new())
    }}
}}
"#)
}

pub fn gen_trait_code(trait_name: &str, s: &str, elem_type: &str, traits: &str) -> String {
    format!(
r#"
trait {trait_name}<{elem_type}> : {traits} {{}}
impl<{elem_type}: 'static + Ord + std::hash::Hash> {trait_name}<{elem_type}> for <{s}<{elem_type}> as ContainerConstructor>::Impl {{}}
"#)
}

// #[cfg(test)]
// mod tests {
//     use crate::generator::{readfile, mark_src_blocks};
//     use crate::parser::{Decl, spec};

//     #[test]
//     fn test_read_file() {
//         assert!(
//             readfile("./spec_code/example.rs".to_string()).len() != 0
//         );
//     }

//     #[test]
//     fn test_parse_file() {
//         let f = readfile("./spec_code/example.rs".to_string());
//         assert!(spec::prog(&f).is_ok())
//     }

// }