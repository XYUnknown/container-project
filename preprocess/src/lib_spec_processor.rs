use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

const LIBSPECNAME: &str = "/*LIBSPEC-NAME*";
const LIBSPECNAMEEND: &str = "*ENDLIBSPEC-NAME*/";
const LIBSPEC: &str = "/*LIBSPEC*";
const LIBSPECEND: &str = "*ENDLIBSPEC*/";
const LANGDECL: &str = "#lang rosette\n";

type ErrorMessage = String;

pub fn read_lib_file(filename : String) -> Result<(String, Vec<String>), ErrorMessage> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let trimed_contents = contents.trim().to_string();

    let name_pragmas: Vec<&str> = trimed_contents.matches(LIBSPECNAME).collect();
    let name_end_pragmas : Vec<&str> = trimed_contents.matches(LIBSPECNAMEEND).collect();
    let spec_pragmas: Vec<&str> = trimed_contents.matches(LIBSPEC).collect();
    let spec_end_pragmas : Vec<&str> = trimed_contents.matches(LIBSPECEND).collect();
    if ((name_pragmas.len() != 1) || (name_end_pragmas.len() != 1)) {
        Err("Error, invalid declaration of library specification name.".to_string())
    } else if (spec_pragmas.len() != spec_end_pragmas.len()) {
        return Err("Error, invalid declaration of library specification.".to_string());
    } else {
        let mut specs = String::new();
        let v1: Vec<&str> = trimed_contents.split(LIBSPECNAME).collect();
        let s = v1.get(1).expect("Error, invalid declaration of library specification name.");
        let v2: Vec<&str> = s.split(LIBSPECNAMEEND).collect();
        let spec_name = v2.get(0).unwrap().trim().to_string();
        let s1 = v1.get(0).expect("Unexpected error.");
        let s2 = v2.get(1).expect("Unexpected error.");
        let mut trimed_contents = String::new();
        trimed_contents.push_str(&s1);
        trimed_contents.push_str(&s2);
        let lib_specs = extract_lib_specs(trimed_contents);
        match lib_specs {
            Ok(v) => {
                Ok((spec_name, v))
            },
            Err(e) => Err(e)
        }
    }
}

pub fn extract_lib_specs(src: String) -> Result<Vec<String>, ErrorMessage> {
    let mut result = Vec::<String>::new();
    let mut contents = src.trim();
    while (contents.len() > 0) {
        if (contents.contains(LIBSPEC) && contents.contains(LIBSPECEND)) {
            let v1: Vec<&str> = contents.splitn(2, LIBSPEC).collect();
            let s = v1.get(1).expect("Error, invalid specification.");
            let v2: Vec<&str> = s.splitn(2, LIBSPECEND).collect();
            let spec = v2.get(0).unwrap().trim().to_string();
            result.push(spec);
            contents = v2.get(1).unwrap().trim();
        } else {
            break;
        }
    }
    Ok(result)
}

pub fn write_lib_file(filename : String, contents: Vec<String>) -> Result<(), Error> {
    let path = "./gen_lib_spec/";

    let mut output = fs::File::create(path.to_owned() + &filename)?;
    write!(output, "{}", LANGDECL.to_string())?;
    for i in 0..contents.len() {
        write!(output, "{}", contents[i])?;
    }
    Ok(())
}

pub fn process_lib_spec(filename: String) -> Result<(), ErrorMessage> {
    let result = read_lib_file(filename);
    match result {
        Ok((name, specs)) => {
            let spec_name = name + ".rkt";
            let state = write_lib_file(spec_name, specs);
            if (!state.is_ok()) {
                return Err("Unable to create lib specification file".to_string());
            }
            Ok(())
        },
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib_spec_processor::{read_lib_file};

    #[test]
    fn test_read_lib_file() {
        assert!(
            read_lib_file("./src/library/list.rs".to_string()).is_ok()
        );
    }

}