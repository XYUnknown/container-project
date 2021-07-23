#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "prims.hpp"

/** Data generation */
class GraphFixture: public ::benchmark::Fixture {
public:
    adjacency_list_t adjList;
    // data source : https://sparse.tamu.edu/vanHeukelum
    void SetUp(const ::benchmark::State& st) {
        adjList = generateUndirectedWeightGraph(st.range(0), st.range(1), 100);
    }

    void TearDown(const ::benchmark::State&) {
        adjList.clear();
    }
};

BENCHMARK_DEFINE_F(GraphFixture, BST)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<bool> visited;
        std::vector<vertex_t> connection;
        std::vector<weight_t> value;
        prims<Container<weight_vertex_pair_t, TreeWrapper>>(adjList, visited, connection, value);
    }
}
BENCHMARK_REGISTER_F(GraphFixture, BST)
    ->Unit(benchmark::kMillisecond)
    ->Args({100, 100})
    //->Args({500, 500})
    ->Args({1000, 1000})
    //->Args({5000, 5000})
    ->Args({10000, 10000})
    //->Args({50000, 50000})
    ->Args({100000, 100000})
    ->Args({1000000, 1000000})
    ->Args({10000000, 10000000});

BENCHMARK_DEFINE_F(GraphFixture, PQ_Vec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<bool> visited;
        std::vector<vertex_t> connection;
        std::vector<weight_t> value;
        prims<std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>(adjList, visited, connection, value);
    }
}
BENCHMARK_REGISTER_F(GraphFixture, PQ_Vec)
    ->Unit(benchmark::kMillisecond)
    ->Args({100, 100})
    //->Args({500, 500})
    ->Args({1000, 1000})
    //->Args({5000, 5000})
    ->Args({10000, 10000})
    //->Args({50000, 50000})
    ->Args({100000, 100000})
    ->Args({1000000, 1000000})
    ->Args({10000000, 10000000});

BENCHMARK_DEFINE_F(GraphFixture, PQ_Deque)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<bool> visited;
        std::vector<vertex_t> connection;
        std::vector<weight_t> value;
        prims<std::priority_queue<weight_vertex_pair_t, std::deque<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>(adjList, visited, connection, value);
    }
}
BENCHMARK_REGISTER_F(GraphFixture, PQ_Deque)
    ->Unit(benchmark::kMillisecond)
    ->Args({100, 100})
    //->Args({500, 500})
    ->Args({1000, 1000})
    //->Args({5000, 5000})
    ->Args({10000, 10000})
    //->Args({50000, 50000})
    ->Args({100000, 100000})
    ->Args({1000000, 1000000})
    ->Args({10000000, 10000000});

BENCHMARK_DEFINE_F(GraphFixture, SortedVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<bool> visited;
        std::vector<vertex_t> connection;
        std::vector<weight_t> value;
        prims<Container<weight_vertex_pair_t, std::vector, Sorted<weight_vertex_pair_t, std::greater<weight_vertex_pair_t>>>>(adjList, visited, connection, value);
    }
}
BENCHMARK_REGISTER_F(GraphFixture, SortedVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({100, 100})
    //->Args({500, 500})
    ->Args({1000, 1000})
    //->Args({5000, 5000})
    ->Args({10000, 10000})
    //->Args({50000, 50000})
    ->Args({100000, 100000})
    ->Args({1000000, 1000000})
    ->Args({10000000, 10000000});

BENCHMARK_MAIN();