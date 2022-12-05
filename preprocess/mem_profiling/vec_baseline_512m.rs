use preprocess::tools::gen_dataset_512;

fn vec_512m() {
    let s = &mut Vec::new();
    let data = gen_dataset_512();
    for val in data.iter() {
        s.push(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    vec_512m();
}