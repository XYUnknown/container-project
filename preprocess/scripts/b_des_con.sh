#!/bin/bash
TIMEFORMAT='
semantic property: descending
syntacyic property: Container
search time: %R seconds
'
time {
cargo run b_des_con.rs gen_des_con.rs
}
