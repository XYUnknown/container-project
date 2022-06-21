#!/bin/bash
TIMEFORMAT='
semantic property: unique
syntactic property: Container
search time: %R seconds
'
time {
cargo run b_unique_con.rs gen_unique_con.rs
}
