#!/bin/bash
TIMEFORMAT='
semantic property: lifo
syntactic property: Container, Stack
search time: %R seconds
'
time {
cargo run b_stack_con.rs gen_stack_con.rs
}
