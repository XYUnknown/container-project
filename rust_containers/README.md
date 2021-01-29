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
| Not sorted and allow dupilcation |[std Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)|[std LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)|       |
| Not sorted and no duplication    |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L75)|[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/unique_linked_list.rs), [tests](https://github.com/XYUnknown/container-project/blob/1b6cae47e7006adec605625198e93b4e53423d15/rust_containers/src/lib.rs#L332)|       |
| Sorted and allow duplication     |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L238)|[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/sorted_linked_list.rs), [tests](https://github.com/XYUnknown/container-project/blob/1b6cae47e7006adec605625198e93b4e53423d15/rust_containers/src/lib.rs#L391)|       |
| Sorted and no duplication        |[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/unique_sorted_vector.rs), [tests](https://github.com/XYUnknown/container-project/blob/7cfc256445a2925d2b031b00e221d1d8c559ea1a/rust_containers/src/lib.rs#L285)|[implementation](https://github.com/XYUnknown/container-project/blob/main/rust_containers/src/unique_sorted_linked_list.rs), [tests](https://github.com/XYUnknown/container-project/blob/1b6cae47e7006adec605625198e93b4e53423d15/rust_containers/src/lib.rs#L467)|       |

## Operations
|         | Vec | UniqueVec | SortedVec | UniqueSortedVec | LinkedList | UniqueLinkedList | SortedLinkedList | UniqueSortedLinkedList | AVL Tree | Unique AVL Tree |
|---------|-----|-----------|-----------|-----------------|------------|------------------|------------------|------------------------|----------|-----------------|
|   new   | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|with_capacity| YES |  YES  |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|from_vec | YES |    YES    |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|   len   | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|is_empty | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|capacity | YES |    YES    |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|   push  | YES |    YES    |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|push_front| NO |    NO     |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|push_back| NO  |    NO     |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|   pop   | YES |    YES    |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|pop_front| NO  |    NO     |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|pop_back | NO  |    NO     |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|insert(by index)| YES |  YES  |  NO    |       NO        |    NO      |        NO        |        NO        |           NO           |          |                 |
|remove(by index)| YES |  YES  |  YES   |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|truncate | YES |    YES    |    YES    |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|  clear  | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|  append | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|first(front)| YES |   YES  |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|last(back)| YES |   YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|get(by index)| YES |  YES  |  YES      |       YES       |    NO      |        NO        |        NO        |           NO           |          |                 |
|  iter   | YES |    YES    |    YES    |       YES       |    YES     |        YES       |        YES       |           YES          |          |                 |
|cursor_front| NO  |    NO  |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|cursor_front_mut| NO  |  NO   |   NO   |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|cursor_back| NO  |    NO   |    NO     |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |
|cursor_back_mut| NO  |  NO    |   NO   |       NO        |    YES     |        YES       |        YES       |           YES          |          |                 |


# Compiler setup
To add some restriction on generic types: `impl<T: PartialEq> UniqueVec<T> `, we need to <[link](https://stackoverflow.com/questions/48593858/how-to-execute-cargo-test-using-the-nightly-channel)>:
- Rust nightly channel required: `rustup install nightly`
- Running tests: `cargo +nightly test`
