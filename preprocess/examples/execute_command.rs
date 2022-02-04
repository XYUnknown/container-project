use preprocess::run_matching::{run, gen_match_script};
fn main() {
    //let result = run("./racket_specs/gen_match/example-match.rkt".to_string());
    //println!("{:?}", result);
    let match_file = gen_match_script("unique".to_string(), "unique.rkt".to_string(), "rust-list-spec.rkt".to_string());
    println!("{:?}", match_file);
}