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
    std::vector<std::string> files = {"../matrix/uw800s.mtx", "../matrix/uw800d.mtx", "../matrix/uw2000s.mtx", "../matrix/uw2000d.mtx", "../matrix/uw5000s.mtx", "../matrix/uw5000d.mtx", "../matrix/uw7000s.mtx", "../matrix/uw7000d.mtx"};
    void SetUp(const ::benchmark::State& st) {
        adjList = constructGraph(files[st.range(0)]);
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
    ->Args({0})
    ->Args({1})
    ->Args({2})
    ->Args({3})
    ->Args({4})
    ->Args({5})
    ->Args({6})
    ->Args({7});

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
    ->Args({0})
    ->Args({1})
    ->Args({2})
    ->Args({3})
    ->Args({4})
    ->Args({5})
    ->Args({6})
    ->Args({7});

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
    ->Args({0})
    ->Args({1})
    ->Args({2})
    ->Args({3})
    ->Args({4})
    ->Args({5})
    ->Args({6})
    ->Args({7});

/*BENCHMARK_DEFINE_F(GraphFixture, UnsortedVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<bool> visited;
        std::vector<vertex_t> connection;
        std::vector<weight_t> value;
        prims<Container<weight_vertex_pair_t, std::vector>>(adjList, visited, connection, value);
    }
}
BENCHMARK_REGISTER_F(GraphFixture, UnsortedVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({0})
    ->Args({1})
    ->Args({2})
    ->Args({3})
    ->Args({4})
    ->Args({5})
    ->Args({6})
    ->Args({7});
*/
BENCHMARK_MAIN();