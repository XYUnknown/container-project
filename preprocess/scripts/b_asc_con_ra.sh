#!/bin/bash
TIMEFORMAT='
semantic property: ascending
syntactic property: Container, RandomAsscess
search time: %R seconds
'
time {
cargo run b_asc_con_ra.rs gen_asc_con_ra.rs
}
