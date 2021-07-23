#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers.hpp"
#include "rng.hpp"

/** Insertion benchmarks */
class InsertionFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size;  // 1KB data
    std::minstd_rand generator;

    void SetUp(const ::benchmark::State& st) {
        size = st.range(0);
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

/* BST (multiset) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, BST_Insertion)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, BST_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/* Sorted vector insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedVector_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted<std::size_t>> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedVector_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}); // 10MB
    /* > 1 min */
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

/* Sorted vector (on access) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessVector_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess<std::size_t>> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
        c.begin(); // access to let it sort
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessVector_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/*  tree set insertion */
BENCHMARK_DEFINE_F(InsertionFixture, TreeSet_Insertion)(benchmark::State& state) {
    Container<std::size_t, TreeSetWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, TreeSet_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/*  hash set insertion */
BENCHMARK_DEFINE_F(InsertionFixture, HashSet_Insertion)(benchmark::State& state) {
    Container<std::size_t, HashSetWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, HashSet_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/* Sorted list insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedList_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::list, Sorted<std::size_t>> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedList_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}); // 1MB
    //->Args({10*128*1024}) // 10MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

/* Sorted list (on access) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessList_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::list, SortedOnAccess<std::size_t>> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
        c.begin(); // access to let it sort
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessList_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_MAIN();