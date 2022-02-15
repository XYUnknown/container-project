#![allow(unused)]
#![feature(map_first_last)]
#![feature(const_btree_new)]
#![feature(linked_list_cursors)]

pub mod parser;
pub mod generator;
pub mod type_check;
pub mod description;
pub mod types;
pub mod analysis;
pub mod inference;
pub mod lib_spec_processor;
pub mod spec_map;
pub mod run_matching;

pub mod library;

pub mod traits;