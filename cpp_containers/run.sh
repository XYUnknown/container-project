#!/bin/bash
mkdir -p results
./bench/benchmark_unique_o2 --benchmark_out=./results/benchmark_unique_o2.json --benchmark_out_format=json
./bench/benchmark_insertion_o2 --benchmark_out=./results/benchmark_insertion_o2.json --benchmark_out_format=json
./bench/benchmark_lookup_o2 --benchmark_out=./results/benchmark_lookup_o2.json --benchmark_out_format=json
./bench/benchmark_insertion_lookup_o2 --benchmark_out=./results/benchmark_insertion_lookup_o2.json --benchmark_out_format=json
