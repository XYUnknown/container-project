use std::collections::{HashSet};
use preprocess::traits::Container;
use preprocess::tools::gen_dataset_128;


fn hashset_insertion_128m() {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_128();
    for val in data.iter() {
        s.insert(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    hashset_insertion_128m();
}