use preprocess::generator::{run};

fn main() {
    println!("{:?}", run("./spec_code/example.rs".to_string(), "example_output.rs".to_string()));
    //println!("{:?}", run("./spec_code/example_unique.rs".to_string(), "example_unique_output.rs".to_string()));
}