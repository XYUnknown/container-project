use preprocess::parser::{spec};
use preprocess::generator::{readfile};

fn main() {
    let f = readfile("./spec_code/example.rs".to_string());
    println!("{:?}", spec::prog(&f));
}