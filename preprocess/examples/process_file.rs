use preprocess::parser::{Decl, spec};
use preprocess::generator::{readfile, writefile, process_src};
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    //let f = readfile("./spec_code/example.rs".to_string());
    //println!("{:?}", spec::prog(&f));
    //println!("{}", type_of(spec::prog(&f)));
    //println!("{}", process_spec(Vec::<Decl>::new()));
    println!("{}", process_src("./spec_code/example.rs".to_string()));
    writefile("example_output_hashset.rs".to_string(), process_src("./spec_code/example.rs".to_string()));

}