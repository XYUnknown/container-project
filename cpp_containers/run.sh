#!/bin/bash
mkdir -p results
./bench/build/benchmark_unique_o2 --benchmark_out=./results/benchmark_unique_o2.json --benchmark_out_format=json
./bench/build/benchmark_insertion_o2 --benchmark_out=./results/benchmark_insertion_o2.json --benchmark_out_format=json
./bench/build/benchmark_lookup_o2 --benchmark_out=./results/benchmark_lookup_o2.json --benchmark_out_format=json
./bench/build/benchmark_insertion_lookup_o2 --benchmark_out=./results/benchmark_insertion_lookup_o2.json --benchmark_out_format=json
./bench/build/benchmark_pq_push_o2 --benchmark_out=./results/benchmark_pq_push_o2.json --benchmark_out_format=json
./bench/build/benchmark_pq_pop_o2 --benchmark_out=./results/benchmark_pq_pop_o2.json --benchmark_out_format=json
./bench/build/benchmark_pq_push_pop_o2 --benchmark_out=./results/benchmark_pq_push_pop_o2.json --benchmark_out_format=json
./bench/build/benchmark_pq_cons_o2 --benchmark_out=./results/benchmark_pq_cons_o2.json --benchmark_out_format=json
./bench/build/benchmark_dijkstra_o2 --benchmark_out=./results/benchmark_dijkstra_o2.json --benchmark_out_format=json
