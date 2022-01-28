use preprocess::lib_spec_processor::{process_lib_spec};
fn main() {
    process_lib_spec("./src/library/list.rs".to_string());
    process_lib_spec("./src/library/vector.rs".to_string());
}