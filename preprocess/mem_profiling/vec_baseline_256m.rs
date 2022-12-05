use preprocess::tools::gen_dataset_256;

fn vec_256m() {
    let s = &mut Vec::new();
    let data = gen_dataset_256();
    for val in data.iter() {
        s.push(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    vec_256m();
}