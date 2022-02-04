use preprocess::lib_spec_processor::{process_lib_specs};
fn main() {
    let lib_specs = process_lib_specs("./src/library/".to_string());
    println!("{:?}", lib_specs)
}