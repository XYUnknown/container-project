#![feature(const_fn)] // enabling impl<T: Ord> 
#![allow(unused)]
#![feature(linked_list_cursors)]
#![feature(test)]
//#![feature(min_specialization)]
#![feature(specialization)] //is incomplete

pub mod unique_vector;
pub mod sorted_vector;
pub mod unique_sorted_vector;
pub mod unique_linked_list;
pub mod sorted_linked_list;
pub mod unique_sorted_linked_list;
pub mod binary_search_tree;
pub mod container;
pub mod sub_containers;
pub mod container_specialization;
pub mod container_specialization_no_wrapper;