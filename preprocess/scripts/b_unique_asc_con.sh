#!/bin/bash
TIMEFORMAT='
semantic property: unique, ascending
syntacyic property: Container
search time: %R seconds
'
time {
cargo run b_unique_asc_con.rs gen_unique_asc_con.rs
}
