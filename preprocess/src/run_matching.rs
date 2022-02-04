use std::process::Command;
use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};


type ExecutionError = String;

const LANGDECL: &str = "#lang rosette\n";
const GENNAME: &str = "./racket_specs/gen_match/match-script.rkt";
const LIBSPECPATH: &str = "../gen_lib_spec/";
const PROPSPECPATH: &str = "../gen_prop_spec/";
const SETUP: &str = "(require \"../match-setup.rkt\")\n";

pub fn gen_match_script(prop: String, prop_spec_file: String, lib_spec_file: String) -> Result<String, Error>  {
    let mut output = fs::File::create(GENNAME.to_owned())?;
    write!(output, "{}", LANGDECL.to_string())?;
    let require_prop = "(require \"".to_string() + PROPSPECPATH + &prop_spec_file + "\")\n";
    write!(output, "{}", require_prop)?;
    let require_lib = "(require \"".to_string() + LIBSPECPATH + &lib_spec_file + "\")\n";
    write!(output, "{}", require_lib)?;
    write!(output, "{}", SETUP.to_string())?;
    let code = "(check ".to_string() + &prop + " pres specs ls elem)\n";
    write!(output, "{}", code)?;
    Ok(GENNAME.to_string())
}

pub fn run(filename: String) -> Result<bool, ExecutionError> {
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