use std::process::Command;
use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

use crate::spec_map::{MatchSetup};

type ExecutionError = String;

const LANGDECL: &str = "#lang rosette\n";
const GENNAME: &str = "./racket_specs/gen_match/match-script.rkt";
const LIBSPECPATH: &str = "../gen_lib_spec/";
const PROPSPECPATH: &str = "../gen_prop_spec/";
//const SETUP: &str = "(require \"../match-setup.rkt\")\n";
const LIBDIR: &str =  "./racket_specs/gen_lib_spec/";
const PROPDIR: &str =  "./racket_specs/gen_prop_spec/";
const MATCHDIR: &str =  "./racket_specs/gen_match/";

pub fn initialise_match_setup() -> MatchSetup {
    let mut match_setup = MatchSetup::new();
    match_setup.insert("Container".to_string(), "../container-setup.rkt".to_string());
    match_setup.insert("RandomAccess".to_string(), "../randomaccess-setup.rkt".to_string());
    match_setup
}


pub fn gen_match_script(prop: String, match_setup: String, prop_spec_file: String, lib_spec_file: String, interface_spec: String) -> Result<String, Error>  {
    let mut output = fs::File::create(GENNAME.to_owned())?;
    write!(output, "{}", LANGDECL.to_string())?;
    let require_prop = "(require \"".to_string() + PROPSPECPATH + &prop_spec_file + "\")\n";
    write!(output, "{}", require_prop)?;
    let require_lib = "(require \"".to_string() + LIBSPECPATH + &lib_spec_file + "\")\n";
    write!(output, "{}", require_lib)?;
    write!(output, "{}", "(require \"".to_string() + &match_setup + "\")\n")?;
    let code = "(check ".to_string() + &prop + " (cdr " + &interface_spec +") (car " + &interface_spec + ") ls n)\n";
    write!(output, "{}", code)?;
    Ok(GENNAME.to_string())
}

pub fn run_matching(filename: String) -> Result<bool, ExecutionError> {
    let output = Command::new("sh")
                .arg("-c")
                .arg("racket ".to_owned() + &filename)
                .output()
                .expect("failed to execute process");
    let raw = output.stdout;
    let result_str = String::from_utf8_lossy(&raw).to_string();
    let result = result_str.trim();
    if (result == "#t") {
        Ok(true)
    } else if (result == "#f"){
        Ok(false)
    } else {
        Err("Error: Not a valid output.".to_string())
    }
}

pub fn cleanup_script() {
    Command::new("sh")
        .arg("-c")
        .arg("rm -f ".to_owned() + GENNAME)
        .output()
        .expect("Fail to clean up");
}

pub fn setup_dirs() {
    Command::new("sh")
        .arg("-c")
        .arg("mkdir -p ".to_owned() + PROPDIR)
        .output()
        .expect("Fail to create the property specification directory");

    Command::new("sh")
        .arg("-c")
        .arg("mkdir -p ".to_owned() + LIBDIR)
        .output()
        .expect("Fail to create the library specification directory");
    
    Command::new("sh")
        .arg("-c")
        .arg("mkdir -p ".to_owned() + MATCHDIR)
        .output()
        .expect("Fail to create the matching script directory");
}