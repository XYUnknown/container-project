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
| Not sorted and allow dupilcation |std Vec|       |       |
| Not sorted and no duplication    |[implementation](https://github.com/XYUnknown/container-project/blob/ea14b8fd2ae72b904885d17fbf94530ab74245be/rust_containers/src/vectors.rs#L5), [test](https://github.com/XYUnknown/container-project/blob/ea14b8fd2ae72b904885d17fbf94530ab74245be/rust_containers/src/lib.rs#L7)|       |       |
| Sorted and allow duplication     |       |       |       |
| Sorted and no duplication        |       |       |       |

# Compiler setup
To add some restriction on generic types: `impl<T: PartialEq> UniqueVec<T> `, we need to <[link](https://stackoverflow.com/questions/48593858/how-to-execute-cargo-test-using-the-nightly-channel)>:
- Rust nightly channel required: `rustup install nightly`
- Running tests: `cargo +nightly test`