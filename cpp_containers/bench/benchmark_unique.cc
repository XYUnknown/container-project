#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers.hpp"
#include "rng.hpp"

/**
 * Given a vector, the performance of different implementation of removing its duplicated items
 */

/** Data generation */
class UniqueFixture: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::vector<std::size_t> data;
    void SetUp(const ::benchmark::State& st) {
        data = Generate(st.range(0));
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

// std sort and unique
BENCHMARK_DEFINE_F(UniqueFixture, STD_Sort_Unique)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::sort(data.begin(), data.end());
        data.erase(std::unique(data.begin(), data.end()), data.end());
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, STD_Sort_Unique)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

// using a hashset
BENCHMARK_DEFINE_F(UniqueFixture, HashSet_Remove_Dup)(benchmark::State& state) {
    Container<std::size_t, HashSetWrapper> c;
    while (state.KeepRunning()) {
        for (size_t e : data)
            c.insert(e);
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, HashSet_Remove_Dup)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

// using a treeset
BENCHMARK_DEFINE_F(UniqueFixture, TreeSet_Remove_Dup)(benchmark::State& state) {
    Container<std::size_t, TreeSetWrapper> c;
    while (state.KeepRunning()) {
        for (size_t e : data)
            c.insert(e);
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, TreeSet_Remove_Dup)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

// using a unique vector
BENCHMARK_DEFINE_F(UniqueFixture, Vector_Remove_Dup)(benchmark::State& state) {
    Container<std::size_t, std::vector, Unique> c;
    while (state.KeepRunning()) {
        for (size_t e : data)
            c.insert(e);
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, Vector_Remove_Dup)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}); // 10MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

// using a sorted unique vector
BENCHMARK_DEFINE_F(UniqueFixture, Vector_Sorted_Unique_Remove_Dup)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted<std::size_t>, Unique> c;
    while (state.KeepRunning()) {
        for (size_t e : data)
            c.insert(e);
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, Vector_Sorted_Unique_Remove_Dup)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}); // 10MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

// using a unique list
BENCHMARK_DEFINE_F(UniqueFixture, List_Remove_Dup)(benchmark::State& state) {
    Container<std::size_t, std::list, Unique> c;
    while (state.KeepRunning()) {
        for (size_t e : data)
            c.insert(e);
    }
}
BENCHMARK_REGISTER_F(UniqueFixture, List_Remove_Dup)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}); // 10MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

BENCHMARK_MAIN();