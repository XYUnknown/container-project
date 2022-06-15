#!/bin/bash
TIMEFORMAT='
semantic property: descending
syntacyic property: Container, RandomAccess
search time: %R seconds
'
time {
cargo run b_des_con_ra.rs gen_des_con_ra.rs
}
