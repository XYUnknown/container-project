#!/bin/bash
TIMEFORMAT='
semantic property: unique, ascending
syntacyic property: Container, RandomAccess
search time: %R seconds
'
time {
cargo run b_unique_asc_con_ra.rs gen_unique_asc_con_ra.rs
}
