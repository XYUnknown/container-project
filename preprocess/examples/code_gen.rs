use preprocess::generator::{gen_output_code};
fn main() {
    println!("{}", gen_output_code("UniqueCon<T>", "T", "std::collections::BTreeSet::<T>", "trait UniqueConTrait<T> : preprocess::traits::container::Container<T>", "UniqueConTrait<T>", "none"));
}