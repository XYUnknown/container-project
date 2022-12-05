use std::collections::{BTreeSet};
use preprocess::traits::Container;
use preprocess::tools::gen_dataset_1;


fn btreeset_insertion_1m() {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_1();
    for val in data.iter() {
        s.insert(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    btreeset_insertion_1m();
}