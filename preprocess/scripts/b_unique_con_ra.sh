#!/bin/bash
TIMEFORMAT='
semantic property: unique
syntacyic properties: Container, RandomAccess
search time: %R seconds
'
time {
cargo run b_unique_con_ra.rs gen_unique_con_ra.rs
}
