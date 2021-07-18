#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers.hpp"
#include "rng.hpp"

/** Helper */
template<class P, class ...Ps>
constexpr bool has_property()
{ return std::disjunction_v<std::is_same<P, Ps>...>; }

/** Lookup benchmarks */
template<template<typename...> class C, class ...Ps>
class LookUpFixture: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    //std::size_t size = 128; // 1KB data
    Container<std::size_t, C, Ps...> c;

    //std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    //std::chrono::milliseconds timeout{60000}; // 1 min

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(st.range(1));
        Generate(c, st.range(0));
        if constexpr (has_property<SortedOnAccess<std::size_t>, Ps...>()) {
            c.sort(); // for sorted vectors and sorted lists
        }
    }

    void TearDown(const ::benchmark::State&) {
        lookups.clear();
        c.clear();
    }
};

// Unsorted Vector Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, UnsortedVec_LookUp, std::vector)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, UnsortedVec_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({10*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({100*128*1024, 1000}); // 100MB, 1000 lookup

    /* > 1min */
    //->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Sorted Vector Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, SortedOnAccessVec_LookUp, std::vector, SortedOnAccess<std::size_t>)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, SortedOnAccessVec_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({10*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Binary Search Tree (Multiset) Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, BST_LookUp, TreeWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, BST_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({10*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// TreeSet Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, TreeSet_LookUp, TreeSetWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, TreeSet_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({10*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// HashSet Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, HashSet_LookUp, HashSetWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, HashSet_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 10MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 10MB, 10000 lookup
    ->Args({10*128*1024, 100000}) // 10MB, 100000 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Sorted List Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, SortedList_LookUp, std::list, SortedOnAccess<std::size_t>)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, SortedList_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}); // 1MB, 10000 lookup

    /* > 1 min */
    //->Args({128*1024, 100000}) // 1MB, 100000 lookup
    //->Args({10*128*1024, 1000}) // 10MB, 1000 lookup
    //->Args({10*128*1024, 10000}) // 10MB, 10000 lookup
    //->Args({10*128*1024, 100000}) // 10MB, 100000 lookup
    //->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    //->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Unsorted List Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, UnsortedList_LookUp, std::list)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, UnsortedList_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 10000}) // 1KB, 10000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({10*128, 1000}) // 10KB, 1000 lookup
    ->Args({10*128, 10000}) // 10KB, 10000 lookup
    ->Args({10*128, 100000}) // 10KB, 100000 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 10000}) // 100KB, 10000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 10000}) // 1MB, 10000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({10*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({10*128*1024, 10000}) // 100MB, 10000 lookup
    //->Args({10*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({100*128*1024, 1000}); // 100MB, 1000 lookup
    //->Args({100*128*1024, 10000}) // 100MB, 10000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 10000}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

BENCHMARK_MAIN();