use preprocess::tools::gen_dataset_128;

fn vec_128m() {
    let s = &mut Vec::new();
    let data = gen_dataset_128();
    for val in data.iter() {
        s.push(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    vec_128m();
}