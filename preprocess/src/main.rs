use preprocess::generator::{run};
use std::env;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { // skip the first arg
        println!("{:?}", run("./spec_code/example.rs".to_string(), "example_output.rs".to_string()));
        Ok(())    
    } else if args.len() == 3 { // skip the first arg
        println!("{:?}", run("./spec_code/".to_string() + args.get(1).unwrap(), args.get(2).unwrap().to_string()));
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Invalid source code paths"))
    }
}