#!/bin/bash
TIMEFORMAT='
semantic property: ascending
syntacyic property: Container
search time: %R seconds
'
time {
cargo run b_asc_con.rs gen_asc_con.rs
}
