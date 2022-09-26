use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use std::mem::size_of;
use std::collections::{BTreeSet, HashSet};
use std::vec::Vec;
use preprocess::traits::{Container};
use preprocess::library::eager_unique_vector::{EagerUniqueVec};
use preprocess::library::lazy_unique_vector::{LazyUniqueVec};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

// one search at the 
fn gen_dataset_1() -> Vec<u32> { // avoid duplication
    let size = 1024 * 1024; // 1 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect(); //ensure no duplication
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

fn gen_dataset_128() -> Vec<u32> { // avoid duplication
    let size = 128 * 1024 * 1024; // 128 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect(); //ensure no duplication
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

fn gen_dataset_256() -> Vec<u32> { // avoid duplication
    let size = 256 * 1024 * 1024; // 256 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect(); //ensure no duplication
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

fn gen_dataset_512() -> Vec<u32> { // avoid duplication
    let size = 512 * 1024 * 1024; // 512 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect(); //ensure no duplication
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

fn btreeset_insertion_1m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_1();
    c.bench_function("btreeset insertion 1MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn btreeset_insertion_128m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_128();
    c.bench_function("btreeset insertion 128MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn btreeset_insertion_256m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_256();
    c.bench_function("btreeset insertion 256MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn btreeset_insertion_512m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_512();
    c.bench_function("btreeset insertion 512MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn hashset_insertion_1m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_1();
    c.bench_function("hashset insertion 1MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn hashset_insertion_128m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_128();
    c.bench_function("hashset insertion 128MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn hashset_insertion_256m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_256();
    c.bench_function("hashset insertion 256MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn hashset_insertion_512m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_512();
    c.bench_function("hashset insertion 512MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn lazy_uniuqe_vec_insertion_1m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut LazyUniqueVec::new();
    let data = gen_dataset_1();
    c.bench_function("lazy unique vector insertion 1MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn lazy_uniuqe_vec_insertion_128m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut LazyUniqueVec::new();
    let data = gen_dataset_128();
    c.bench_function("lazy unique vector insertion 128MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn lazy_uniuqe_vec_insertion_256m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut LazyUniqueVec::new();
    let data = gen_dataset_256();
    c.bench_function("lazy unique vector insertion 256MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}

fn lazy_uniuqe_vec_insertion_512m(c: &mut Criterion) {
    let s: &mut dyn Container<u32> = &mut LazyUniqueVec::new();
    let data = gen_dataset_512();
    c.bench_function("lazy unique vector insertion 512MB", |b| b.iter(|| {
        for val in data.iter() {
            s.insert(*val);
        }
        s.contains(&1024);
    }
    )); 
}


criterion_group! {
    name = insertion_1m;
    config = Criterion::default().sample_size(10);
    targets = btreeset_insertion_1m, hashset_insertion_1m, lazy_uniuqe_vec_insertion_1m
}

criterion_group! {
    name = insertion_128m;
    config = Criterion::default().sample_size(10);
    targets = btreeset_insertion_128m, hashset_insertion_128m, lazy_uniuqe_vec_insertion_128m
}

criterion_group! {
    name = insertion_256m;
    config = Criterion::default().sample_size(10);
    targets = btreeset_insertion_256m, hashset_insertion_256m, lazy_uniuqe_vec_insertion_256m
}

criterion_group! {
    name = insertion_512m;
    config = Criterion::default().sample_size(10);
    targets = btreeset_insertion_512m, hashset_insertion_512m, lazy_uniuqe_vec_insertion_512m
}

criterion_main!(insertion_1m, insertion_128m, insertion_256m, insertion_512m);
