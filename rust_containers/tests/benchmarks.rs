#![feature(test)]

use rust_containers;
use rust_containers::binary_search_tree::BinarySearchTree;
use rust_containers::sorted_vector::SortedVec;
use rust_containers::sorted_vector::SortedVecAlt;
use std::collections::BTreeSet;

extern crate test;
use test::Bencher;

#[bench]
fn bst_insertion(b: &mut Bencher) {
    let mut t = BinarySearchTree::new();
    b.iter(|| for x in 0..10000 {
        t.insert(x);
    });
}

#[bench]
fn bst_contains(b: &mut Bencher) {
    let mut t = BinarySearchTree::new();
    for x in 0..10000 {
        t.insert(x);
    }
    b.iter(|| t.contains(1));
}

#[bench]
fn bst_remove(b: &mut Bencher) {
    let mut t = BinarySearchTree::new();
    for x in 0..10000 {
        t.insert(x);
    }
    b.iter(|| t.remove(5));
}

#[bench]
fn btreeset_insertion(b: &mut Bencher) {
    let mut t = BTreeSet::new();
    b.iter(|| for x in 0..10000 {
        t.insert(x);
    });
}

#[bench]
fn btreeset_contains(b: &mut Bencher) {
    let mut t = BTreeSet::new();
    for x in 0..10000 {
        t.insert(x);
    }
    b.iter(|| t.contains(&1));
}

#[bench]
fn btreeset_remove(b: &mut Bencher) {
    let mut t = BTreeSet::new();
    for x in 0..10000 {
        t.insert(x);
    }
    b.iter(|| t.remove(&5));
}

#[bench]
fn sorted_vec_many_push_one_pop(b: &mut Bencher) {
    let mut vec = SortedVec::new();
    b.iter(|| {
        for x in 0..10000 {
            vec.push(x);
        }
        vec.pop();
    });
}

#[bench]
fn sorted_vec_alt_many_push_one_pop(b: &mut Bencher) {
    let mut vec = SortedVecAlt::new();
    b.iter(|| {
        for x in 0..10000 {
            vec.push(x);
        }
        vec.pop();
    });
}