#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "prims.hpp"

int getEdgeNum(int v, double density) {
    return v * (v-1) / 2 * density; 
}

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
    ->Args({100, getEdgeNum(100, 0.7)})
    ->Args({500, getEdgeNum(500, 0.7)})
    ->Args({1000, getEdgeNum(1000, 0.7)})
    ->Args({5000, getEdgeNum(5000, 0.7)});

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
    ->Args({100, getEdgeNum(100, 0.7)})
    ->Args({500, getEdgeNum(500, 0.7)})
    ->Args({1000, getEdgeNum(1000, 0.7)})
    ->Args({5000, getEdgeNum(5000, 0.7)});

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
    ->Args({100, getEdgeNum(100, 0.7)})
    ->Args({500, getEdgeNum(500, 0.7)})
    ->Args({1000, getEdgeNum(1000, 0.7)})
    ->Args({5000, getEdgeNum(5000, 0.7)});

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
    ->Args({100, getEdgeNum(100, 0.7)})
    ->Args({500, getEdgeNum(500, 0.7)})
    ->Args({1000, getEdgeNum(1000, 0.7)})
    ->Args({5000, getEdgeNum(5000, 0.7)});
    
BENCHMARK_MAIN();