# The Tool For Selecting Container Implementations According to Property Specifications
## Dependencies
- Rust 1.60.0-nightly: [installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Rosette: [installation guide](https://docs.racket-lang.org/rosette-guide/ch_getting-started.html)
  - We used the Z3 backend in this project

## Executing the tool
- Make sure the program with property specifications is put under the directory `./spec_code/`
- Run the tool with command:
```
cargo run [input.rs] [output.rs]
```
for example
```
cargo run example_unique.rs example_unique_output.rs
```
- The generated file will appear in the directory `./gen_code/`

