#include <benchmark/benchmark.h>
#include <queue>
#include <vector>
#include <set>
#include <deque>
#include <functional>

#include "containers.hpp"
#include "rng.hpp"

/** Data generation */
class PQFixture: public ::benchmark::Fixture {
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

BENCHMARK_DEFINE_F(PQFixture, Vec_PQ_Construction)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::size_t, std::vector<std::size_t>> q(data.begin(), data.end());
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ_Construction)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ_Construction)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::size_t, std::deque<std::size_t>> q(data.begin(), data.end());
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ_Construction)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Vec_PQ_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::size_t, std::vector<std::size_t>> q;
        for (std::size_t e: data)
            q.push(e);
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::size_t, std::deque<std::size_t>> q;
        for (std::size_t e: data)
            q.push(e);
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Vec_SortedOnAccess_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::size_t, std::vector, SortedOnAccess<std::size_t>> q;
        for (std::size_t e: data)
            q.insert(e);
        q.sort();
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_SortedOnAccess_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({10*128}) // 10KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({10*128*1024}) // 10MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

BENCHMARK_MAIN();