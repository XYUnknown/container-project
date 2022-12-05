use std::mem::size_of;
use std::collections::{BTreeSet, HashSet};
use std::vec::Vec;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn gen_dataset_1() -> Vec<u32> {
    let size = 1024 * 1024; // 1 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect();
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

pub fn gen_dataset_128() -> Vec<u32> {
    let size = 128 * 1024 * 1024; // 128 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect();
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

pub fn gen_dataset_256() -> Vec<u32> {
    let size = 256 * 1024 * 1024; // 256 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect();
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}

pub fn gen_dataset_512() -> Vec<u32> {
    let size = 512 * 1024 * 1024; // 512 MB
    let amount = size / size_of::<u32>();
    let mut data: Vec<u32> = (1..amount as u32).collect();
    let mut rng = StdRng::seed_from_u64(222);
    data.shuffle(&mut rng);
    data
}