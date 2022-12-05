use preprocess::tools::gen_dataset_1;

fn vec_1m() {
    let s = &mut Vec::new();
    let data = gen_dataset_1();
    for val in data.iter() {
        s.push(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    vec_1m();
}