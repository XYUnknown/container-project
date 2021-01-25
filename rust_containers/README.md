# Container Types and Properties We Would Like to Study in Current Stage

## Containers
- Vector
- List
- Binary Tree (AVL)

## Properties
- Affects how the interface should look like
    - Indexed/Not Indexed
- Does not affect how the interface should look like but affects how the implementation details of operations on containers
    - Sorted/Not Sorted
    - Allow duplication/No duplication

## Table for tracking progress
|                | Indexed     |    Not Indexed            ||
| -------------- | ----------- | ----------- | ----------- |
| Container Type |   Vector    |    List     | Binary Tree |
| Not sorted and allow dupilcation |[std Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)|       |       |
| Not sorted and no duplication    |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L75)|       |       |
| Sorted and allow duplication     |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L238)|       |       |
| Sorted and no duplication        |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/unique_sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L285)|       |       |

# Compiler setup
To add some restriction on generic types: `impl<T: PartialEq> UniqueVec<T> `, we need to <[link](https://stackoverflow.com/questions/48593858/how-to-execute-cargo-test-using-the-nightly-channel)>:
- Rust nightly channel required: `rustup install nightly`
- Running tests: `cargo +nightly test`
