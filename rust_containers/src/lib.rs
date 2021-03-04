#![feature(const_fn)] // enabling impl<T: Ord> 
#![allow(unused)]
#![feature(linked_list_cursors)]
#![feature(test)]
//#![feature(min_specialization)]
#![feature(specialization)] //is incomplete

pub mod container_library;
pub mod container_naive;
pub mod container_specialization;