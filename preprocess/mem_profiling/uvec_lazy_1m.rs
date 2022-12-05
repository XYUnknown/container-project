use preprocess::library::lazy_unique_vector::{LazyUniqueVec};
use preprocess::traits::Container;
use preprocess::tools::gen_dataset_1;

fn vec_insertion_1m() {
    let s: &mut dyn Container<u32> = &mut LazyUniqueVec::new();
    let data = gen_dataset_1();
    for val in data.iter() {
        s.insert(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    vec_insertion_1m();
}