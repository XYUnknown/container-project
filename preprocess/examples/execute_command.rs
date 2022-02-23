use preprocess::run_matching::{run_matching, gen_match_script, cleanup_script, setup_dirs};
fn main() {
    //setup_dirs();
    //let result = run_matching("./racket_specs/gen_match/example-match.rkt".to_string());
    //println!("{:?}", result);
    let match_file = gen_match_script("unique".to_string(), "../match-setup.rkt".to_string(), "unique.rkt".to_string(), "rust-list-spec.rkt".to_string(), "container".to_string());
    println!("{:?}", match_file);
    //cleanup_script();
}