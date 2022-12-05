#!/bin/bash
TIMEFORMAT='
Property based tests running time: %R seconds
'
time {
cargo test
}
