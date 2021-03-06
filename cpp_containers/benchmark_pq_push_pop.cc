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

/* push elements into pq and pop to arrange accoring to priority */
BENCHMARK_DEFINE_F(PQFixture, Vec_PQ)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        for (std::pair<std::size_t, std::string> e: data)
            q.push(e);
        while (!q.empty()) {
            results.push_back(q.top());
            q.pop();
            results = results;
        } 
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        for (std::pair<std::size_t, std::string>e: data)
            q.push(e);
        while (!q.empty()) {
            results.push_back(q.top());
            q.pop();
            results = results;
        }
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, SortedOnAccessVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector, SortedOnAccess<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        for (std::pair<std::size_t, std::string> e: data)
            q.insert(e);
        while (!q.empty()) {
            results.push_back(q.back());
            q.pop_back();
            results = results;
        }
    }
}
BENCHMARK_REGISTER_F(PQFixture, SortedOnAccessVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Tree)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, TreeWrapper> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        for (std::pair<std::size_t, std::string> e: data)
            q.insert(e);
        while (!q.empty()) {
            results.push_back(*q.rbegin());
            q.erase(--q.end());
            results = results;
        }
    }
}
BENCHMARK_REGISTER_F(PQFixture, Tree)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, SortedOnAccessList)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::list, SortedOnAccess<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        for (std::pair<std::size_t, std::string> e: data)
            q.insert(e);
        while (!q.empty()) {
            results.push_back(q.back());
            q.pop_back();
            results = results;
        }
    }
}
BENCHMARK_REGISTER_F(PQFixture, SortedOnAccessList)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_MAIN();