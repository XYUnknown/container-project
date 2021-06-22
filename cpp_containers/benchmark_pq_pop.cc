#include <benchmark/benchmark.h>
#include <queue>
#include <vector>
#include <set>
#include <deque>
#include <functional>

#include "containers.hpp"
#include "rng.hpp"

/** Fixture for Container */
template<template<typename...> class C, class ...Ps>
class PopFixtureC: public ::benchmark::Fixture {
public:
    // std::pair<std::size_t, std::string> --> 32 bytes
    Container<std::pair<std::size_t, std::string>, C, Ps...> c;

    void SetUp(const ::benchmark::State& st) {
        generate_pairs_c(c, 5, st.range(0));
    }

    void TearDown(const ::benchmark::State&) {
        c.clear();
    }
};

/** Fixture for std pq */
template<class PQ>
class PopFixturePQ: public ::benchmark::Fixture {
public:
    // std::pair<std::size_t, std::string> --> 32 bytes
    PQ q;

    void SetUp(const ::benchmark::State& st) {
        generate_pairs_pq(q, 5, st.range(0));
    }

    void TearDown(const ::benchmark::State&) {
        q = PQ();
    }
};

BENCHMARK_TEMPLATE_DEFINE_F(PopFixturePQ, Vec_PQ_Pop, std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        while (!q.empty()) {
            results.push_back(q.top());
            q.pop();
        }        
    }
}
BENCHMARK_REGISTER_F(PopFixturePQ, Vec_PQ_Pop)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_TEMPLATE_DEFINE_F(PopFixturePQ, Deque_PQ_Pop, std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        while (!q.empty()) {
            results.push_back(q.top());
            q.pop();
        }        
    }
}
BENCHMARK_REGISTER_F(PopFixturePQ, Deque_PQ_Pop)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_TEMPLATE_DEFINE_F(PopFixtureC, Vec_SortedOnAccess_Pop, std::vector, SortedOnAccess)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        while (!c.empty()) {
            results.push_back(c.back());
            c.pop_back();
        }        
    }
}
BENCHMARK_REGISTER_F(PopFixtureC, Vec_SortedOnAccess_Pop)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_TEMPLATE_DEFINE_F(PopFixtureC, Tree_Pop, TreeWrapper)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        while (!c.empty()) {
            results.push_back(*c.rbegin());
            c.erase(--c.end());
        }        
    }
}
BENCHMARK_REGISTER_F(PopFixtureC, Tree_Pop)
    ->Unit(benchmark::kMillisecond)
    ->Args({32}) // 1KB
    ->Args({10*32}) // 10KB
    ->Args({100*32}) // 100KB
    ->Args({32*1024}) // 1MB
    ->Args({10*32*1024}) // 10MB
    ->Args({100*32*1024}) // 100MB
    ->Args({32*1024*1024}); // 1GB

BENCHMARK_MAIN();