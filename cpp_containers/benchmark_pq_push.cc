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
    // std::pair<std::size_t, std::string> --> 32 bytes
    std::vector<std::pair<std::size_t, std::string>> data;
    void SetUp(const ::benchmark::State& st) {
        data = generate_pairs(5, st.range(0)); // length of each string is 5
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

/* constucting pq with using vector */
BENCHMARK_DEFINE_F(PQFixture, Vec_PQ_Construction)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>> q(data.begin(), data.end());
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ_Construction)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ_Construction)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>> q(data.begin(), data.end());
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ_Construction)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB


/* push elements into pq */
BENCHMARK_DEFINE_F(PQFixture, Vec_PQ_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>> q;
        for (std::pair<std::size_t, std::string> e: data)
            q.push(e);
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>> q;
        for (std::pair<std::size_t, std::string>e: data)
            q.push(e);
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Vec_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector, SortedOnAccess> q;
        for (std::pair<std::size_t, std::string> e: data)
            q.insert(e);
        q.sort();
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Tree_Push)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, TreeWrapper> q;
        for (std::pair<std::size_t, std::string> e: data)
            q.insert(e);
    }
}
BENCHMARK_REGISTER_F(PQFixture, Tree_Push)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_MAIN();