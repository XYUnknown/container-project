use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use std::mem::size_of;
use std::collections::BTreeSet;
use std::vec::Vec;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use rust_containers;
use rust_containers::binary_search_tree::BinarySearchTree;
use rust_containers::sorted_vector::SortedVec;
use rust_containers::sorted_vector::SortedVecAlt;
use rust_containers::unique_vector::UniqueVec;

fn gen_dataset() -> Vec<u32> { // avoid duplication
    let size = 12 * 1024 * 1024; // 12 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect(); //ensure no duplication
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

fn ten_sec_measurement() -> Criterion {
    let ten_seconds = Duration::new(10, 0);
    Criterion::default().measurement_time(ten_seconds)
}

fn fifteen_sec_measurement() -> Criterion {
    let fifteen_seconds = Duration::new(15, 0);
    Criterion::default().measurement_time(fifteen_seconds)
}

fn long_measurement() -> Criterion {
    let fifteen_seconds = Duration::new(1000, 0);
    Criterion::default().measurement_time(fifteen_seconds)
}

fn bst_insertion(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    let data = gen_dataset();
    c.bench_function("bst insertion", |b| b.iter(||
        for i in 0..data.len() {
            let val = data.get(i);
            match val {
                Some(x) => t.insert(x),
                None => break
            }
        }
    ));
}

fn bst_contains(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    let data = gen_dataset();
    for i in 0..data.len() {
        let val = data.get(i);
        match val {
            Some(x) => t.insert(*x),
            None => break
        }
    }
    c.bench_function("bst contains", |b| b.iter(|| t.contains(&10000)));
}

fn bst_remove(c: &mut Criterion) {
    let mut t = BinarySearchTree::new();
    let data = gen_dataset();
    for i in 0..data.len() {
        let val = data.get(i);
        match val {
            Some(x) => t.insert(*x),
            None => break
        }
    }
    c.bench_function("bst remove", |b| b.iter(|| t.remove(&10000)));
}

fn btreeset_insertion(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    let data = gen_dataset();
    c.bench_function("btreeset insertion", |b| b.iter(|| 
        for i in 0..data.len() {
            let val = data.get(i);
            match val {
                Some(x) => {t.insert(x);},
                None => break
            }
        }
    )); 
}

fn btreeset_contains(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    let data = gen_dataset();
    for i in 0..data.len() {
        let val = data.get(i);
        match val {
            Some(x) => {t.insert(x);},
            None => break
        }
    }
    c.bench_function("btreeset contains", |b| b.iter(|| t.contains(&10000)));
}

fn btreeset_remove(c: &mut Criterion) {
    let mut t = BTreeSet::new();
    let data = gen_dataset();
    for i in 0..data.len() {
        let val = data.get(i);
        match val {
            Some(x) => {t.insert(x);},
            None => break
        }
    }
    c.bench_function("btreeset remove", |b| b.iter(|| t.remove(&10000)));
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

criterion_group! {
    name = tree_benches;
    config = long_measurement();
    targets = bst_insertion, bst_contains, bst_remove, btreeset_insertion, btreeset_contains, btreeset_remove
}

criterion_group! {
    name = vec_benches;
    config = fifteen_sec_measurement();
    targets = sorted_vec_many_push_one_pop, sorted_vec_alt_many_push_one_pop
}
criterion_main!(tree_benches, vec_benches);
