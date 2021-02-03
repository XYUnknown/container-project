use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use rust_containers;
use rust_containers::binary_search_tree::BinarySearchTree;
use rust_containers::sorted_vector::SortedVec;
use rust_containers::sorted_vector::SortedVecAlt;
use std::collections::BTreeSet;

fn ten_sec_measurement() -> Criterion {
    let ten_seconds = Duration::new(10, 0);
    Criterion::default().measurement_time(ten_seconds)
}

fn fifteen_sec_measurement() -> Criterion {
    let fifteen_seconds = Duration::new(15, 0);
    Criterion::default().measurement_time(fifteen_seconds)
}

fn bst_insertion(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    c.bench_function("bst insertion", |b| b.iter(|| for x in 0..10000 {
        t.insert(x);
    }));
}

fn bst_contains(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    for x in 0..10000 {
        t.insert(x);
    }
    c.bench_function("bst contains", |b| b.iter(|| t.contains(1)));
}

fn bst_remove(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    for x in 0..10000 {
        t.insert(x);
    }
    c.bench_function("bst remove", |b| b.iter(|| t.remove(5)));
}

fn btreeset_insertion(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    c.bench_function("btreeset insertion", |b| b.iter(|| for x in 0..10000 {
        t.insert(x);
    })); 
}

fn btreeset_contains(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    for x in 0..10000 {
        t.insert(x);
    }
    c.bench_function("btreeset contains", |b| b.iter(|| t.contains(&1)));
}

fn btreeset_remove(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    for x in 0..10000 {
        t.insert(x);
    }
    c.bench_function("btreeset remove", |b| b.iter(|| t.remove(&5)));
}

fn sorted_vec_many_push_one_pop(c: &mut Criterion) {
    let mut vec = SortedVec::new();
    c.bench_function("sorted vec many push one pop", |b| b.iter(|| {
        for x in 0..10000 {
            vec.push(x);
        }
        vec.pop();
    }));
}

fn sorted_vec_alt_many_push_one_pop(c: &mut Criterion) {
    let mut vec = SortedVecAlt::new();
    c.bench_function("sorted vec alter many push one pop", |b| b.iter(|| {
        for x in 0..10000 {
            vec.push(x);
        }
        vec.pop();
    }));
}

criterion_group!(five_sec_benches, bst_contains, bst_remove, btreeset_insertion, btreeset_contains, btreeset_remove);
criterion_group! {
    name = ten_sec_benches;
    config = ten_sec_measurement();
    targets = bst_insertion
}

criterion_group! {
    name = vec_benches;
    config = fifteen_sec_measurement();
    targets = sorted_vec_many_push_one_pop, sorted_vec_alt_many_push_one_pop
}
criterion_main!(ten_sec_benches, five_sec_benches, vec_benches);
