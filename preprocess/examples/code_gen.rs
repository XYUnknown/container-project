use preprocess::generator::{gen_output_code};
fn main() {
    println!("{}", gen_output_code("UniqueCon", "T", "std::collections::BTreeSet", "UniqueConTrait", "none"));
}