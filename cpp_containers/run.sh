#!/bin/bash
mkdir -p results

./build/benchmark_o2 --benchmark_out=./results/benchmark_o2.json --benchmark_out_format=json

./build/benchmark_o3 --benchmark_out=./results/benchmark_o3.json --benchmark_out_format=json