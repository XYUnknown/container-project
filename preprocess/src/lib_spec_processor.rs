use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};
use std::collections::BTreeMap;
use std::collections::btree_map::Iter;
//use std::collections::hash_map::Iter;
use std::collections::HashMap;

use crate::spec_map::{LibSpecs, Bounds, ProvidedOps};

const LIBSPECNAME: &str = "/*LIBSPEC-NAME*";
const LIBSPECNAMEEND: &str = "*ENDLIBSPEC-NAME*/";
const LIBSPEC: &str = "/*LIBSPEC*";
const LIBSPECEND: &str = "*ENDLIBSPEC*/";
const LANGDECL: &str = "#lang rosette\n";
const GENPATH: &str = "./racket_specs/gen_lib_spec/";
const OPNAME: &str = "/*OPNAME*";
const OPNAMEEND: &str = "*ENDOPNAME*/";
const IMPL: &str = "/*IMPL*";
const IMPLEND: &str = "*ENDIMPL*/";

type ErrorMessage = String;

fn is_next_pragma_impl(src: &String) -> bool {
    match src.find(IMPL) {
        Some(impl_pos) => {
            match src.find(LIBSPEC) {
                Some(spec_pos) => {
                    impl_pos < spec_pos
                },
                None => true
            }
        },
        None => false
    }
}

fn has_pragma_impl(src: &String) -> bool {
    src.contains(IMPL)
}

fn has_pragma_spec(src: &String) -> bool {
    src.contains(LIBSPEC)
}

pub fn read_lib_file(filename : String) -> Result<(String, String, Vec<String>, String, Bounds, ProvidedOps), ErrorMessage> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let trimed_contents = contents.trim().to_string();

    let name_pragmas: Vec<&str> = trimed_contents.matches(LIBSPECNAME).collect();
    let name_end_pragmas: Vec<&str> = trimed_contents.matches(LIBSPECNAMEEND).collect();
    let spec_pragmas: Vec<&str> = trimed_contents.matches(LIBSPEC).collect();
    let spec_end_pragmas : Vec<&str> = trimed_contents.matches(LIBSPECEND).collect();
    if ((name_pragmas.len() != 1) || (name_end_pragmas.len() != 1)) {
        return Err("Error, invalid declaration of library specification name**.".to_string());
    } else if (spec_pragmas.len() != spec_end_pragmas.len()) {
        return Err("Error, invalid declaration of library specification.".to_string());
    } else {
        let mut specs = String::new();
        let v1: Vec<&str> = trimed_contents.split(LIBSPECNAME).collect();
        let s = v1.get(1).expect("Error, invalid declaration of library specification name.");
        let v2: Vec<&str> = s.split(LIBSPECNAMEEND).collect();
        let s3 = v2.get(0).unwrap().trim().to_string();
        let v3: Vec<&str> = s3.split(" ").collect();
        let spec_name = v3.get(0).unwrap().trim().to_string();
        let struct_name = v3.get(1).unwrap().trim().to_string();
        let s1 = v1.get(0).expect("Unexpected error.");
        let s2 = v2.get(1).expect("Unexpected error.");
        // process interface blocks
        let mut trimed_contents = String::new();
        trimed_contents.push_str(&s1);
        trimed_contents.push_str(&s2);
        if (!is_next_pragma_impl(&trimed_contents) && has_pragma_spec(&trimed_contents)) {
            return Err("Specification without declared interface is not allowed".to_string());
        } else {
            let mut interfaces = Vec::<String>::new();
            let mut interface_info = HashMap::<String, BTreeMap<String, (String, String, String)>>::new();
            let mut code = Vec::<String>::new();
            let mut provided_ops = Vec::<String>::new();
            while (has_pragma_impl(&trimed_contents)) {
                let v4: Vec<&str> = trimed_contents.splitn(2, IMPL).collect();
                let s4 = v4.get(1).expect("Error, invalid interface declaration.");
                let v5: Vec<&str> = s4.splitn(2, IMPLEND).collect();
                let mut interface_name = v5.get(0).unwrap().trim().to_string();
                trimed_contents = v5.get(1).unwrap().trim().to_string();
                let lib_specs = extract_lib_specs(trimed_contents);
                match lib_specs {
                    Ok((rest, mut v, infos, mut ops)) => {
                        code.append(&mut v);
                        interface_info.insert(interface_name.clone(), infos);
                        interfaces.push(interface_name.clone());
                        provided_ops.append(&mut ops);
                        trimed_contents = rest;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            let (provide, interface_provide_map) = generate_provide(interface_info);
            Ok((spec_name, struct_name, code.clone(), provide, interface_provide_map, (code, provided_ops)))
        }
    }
}

pub fn generate_provide(interface_info: HashMap<String, BTreeMap<String, (String, String, String)>>) -> (String, Bounds) {
    let mut interfaces = Vec::<String>::new();
    let mut provide = String::new();
    let mut interface_provide_map = Bounds::new();
    for (interface, infos) in interface_info.iter() {
        let mut specs = Vec::<String>::new();
        let mut pres = Vec::<String>::new();
        for (key, value) in infos.iter() {
            specs.push(value.0.clone());
            pres.push(value.1.clone());
        }
        let specs_name = interface.to_lowercase() + "-specs";
        let pres_name = interface.to_lowercase() + "-pres";
        let interface_name = interface.to_lowercase();
        let specs_str = "\n(define ".to_string() + &specs_name + " (list " + &specs.join(" ") + "))\n";
        let pres_str = "(define ".to_string()  + &pres_name + " (list " + &pres.join(" ") + "))\n";
        let interface_str = "(define ".to_string() + &interface_name + " (cons " + &specs_name + " " + &pres_name + "))\n";
        provide = provide + &specs_str + &pres_str + &interface_str;
        interfaces.push(interface.to_lowercase());
        interface_provide_map.insert(interface.to_string(), interface_name);
    }
    let provide_str = "(provide ".to_string() + &interfaces.join(" ") + ")";

    provide = provide + &provide_str;
    (provide, interface_provide_map)
}

pub fn extract_lib_specs(src: String) -> Result<(String, Vec<String>, BTreeMap<String, (String, String, String)>, Vec<String>), ErrorMessage> {
    let mut result = Vec::<String>::new();
    let mut contents = src.trim();
    let mut op_infos = BTreeMap::<String, (String, String, String)>::new();
    let mut provided_ops = Vec::<String>::new();
    while (contents.len() > 0 && !is_next_pragma_impl(&contents.to_string())) {
        if (contents.contains(LIBSPEC) && contents.contains(LIBSPECEND)) {
            let v1: Vec<&str> = contents.splitn(2, LIBSPEC).collect();
            let s = v1.get(1).expect("Error, invalid specification.");
            let v2: Vec<&str> = s.splitn(2, LIBSPECEND).collect();
            let spec = v2.get(0).unwrap().trim().to_string();
            let info = extract_op_info(spec.clone()).unwrap();
            op_infos.insert(info.0, (info.1.clone(), info.2, info.3));
            provided_ops.push(info.1);
            let v3: Vec<&str> = spec.splitn(2, OPNAMEEND).collect();
            let code = v3.get(1).unwrap().trim_matches(|c| c == '\t' || c == ' ').to_string();
            result.push(code);
            contents = v2.get(1).unwrap().trim();
        } else {
            break;
        }
    }
    Ok((contents.to_string(), result, op_infos, provided_ops))
}

pub fn extract_op_info(spec: String) -> Result<(String, String, String, String), ErrorMessage> {
    let op_name_pragmas: Vec<&str> = spec.matches(OPNAME).collect();
    let op_name_end_pragmas : Vec<&str> = spec.matches(OPNAMEEND).collect();
    if (spec.chars().nth(0).unwrap() == '/' && op_name_pragmas.len()==1 && op_name_end_pragmas.len() == 1) {
        let v1: Vec<&str> = spec.split(OPNAME).collect();
        let s = v1.get(1).expect("Error, invaild operation information declaration.");
        let v2: Vec<&str> = s.split(OPNAMEEND).collect();
        let info_string = v2.get(0).expect("Error, invaild operation information declaration.");
        let mut infos: Vec<&str> = info_string.trim().split(" ").collect();
        if (infos.len() == 4) {
            let post = infos.pop().unwrap();
            let pre = infos.pop().unwrap();
            let op_spec = infos.pop().unwrap();
            let name = infos.pop().unwrap();
            Ok((name.to_string(), op_spec.to_string(), pre.to_string(), post.to_string()))
        } else {
            Err("Error, invaild operation information declaration.".to_string())
        }
    } else {
        Err("Error, invaild operation information declaration.".to_string())
    }
}

pub fn write_lib_file(filename : String, contents: Vec<String>, provide: String) -> Result<(), Error> {
    let path = GENPATH;

    let mut output = fs::File::create(path.to_owned() + &filename)?;
    write!(output, "{}", LANGDECL.to_string())?;
    for i in 0..contents.len() {
        write!(output, "{}", contents[i])?;
    }
    write!(output, "{}", provide)?;
    Ok(())
}

pub fn process_lib_spec(filename: String) -> Result<(String, String, Bounds, ProvidedOps), ErrorMessage> {
    let result = read_lib_file(filename);
    match result {
        Ok((name, struct_name, specs, provide, interface_provide_map, provided_ops)) => {
            let spec_name = name + ".rkt";
            let state = write_lib_file(spec_name.clone(), specs, provide);
            if (!state.is_ok()) {
                return Err("Unable to create lib specification file".to_string());
            }
            Ok((spec_name, struct_name, interface_provide_map, provided_ops))
        },
        Err(e) => Err(e)
    }
}

pub fn process_lib_specs(dirname: String) -> Result<LibSpecs, ErrorMessage> {
    let paths = fs::read_dir(dirname).unwrap();
    let files : Vec<String> = paths.into_iter()
                                        .map(|path| path.unwrap().path().to_str().unwrap().to_string())
                                        .filter(|path| !path.contains("/mod.rs"))
                                        .collect();
    let mut lib_specs = LibSpecs::new();
    for path in files {
        match process_lib_spec(path) {
            Ok((spec_name, struct_name, interface_provide_map, provided_ops)) => {
                lib_specs.insert(struct_name, (spec_name, interface_provide_map, provided_ops));
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(lib_specs)
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