#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "dijkstra.hpp"

/** Data generation */
class GraphFixture: public ::benchmark::Fixture {
public:
    adjacency_list_t adjList;
    std::vector<std::string> files = {"./matrix/cage4.mtx", "./matrix/cage6.mtx", "./matrix/cage8.mtx", "./matrix/cage10.mtx", "./matrix/cage12.mtx", "./matrix/cage14.mtx"};
    void SetUp(const ::benchmark::State& st) {
        adjList = constructGraph(files[st.range(0)]);
    }

    void TearDown(const ::benchmark::State&) {
        adjList.clear();
    }
};

BENCHMARK_DEFINE_F(GraphFixture, BST)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<weight_t> min_distance;
        std::vector<vertex_t> previous;
        DijkstraComputePaths<Container<weight_vertex_pair_t, TreeWrapper>>(0, adjList, min_distance, previous);
        std::list<vertex_t> path = DijkstraGetShortestPathTo(state.range(1), previous);
        path = path;
    }
}
BENCHMARK_REGISTER_F(GraphFixture, BST)
    ->Unit(benchmark::kMillisecond)
    ->Args({0, 4})
    ->Args({1, 40})
    ->Args({2, 400})
    ->Args({3, 4000})
    ->Args({4, 40000})
    ->Args({5, 400000});

BENCHMARK_DEFINE_F(GraphFixture, PQ_Vec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<weight_t> min_distance;
        std::vector<vertex_t> previous;
        DijkstraComputePaths<std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>(0, adjList, min_distance, previous);
        std::list<vertex_t> path = DijkstraGetShortestPathTo(state.range(1), previous);
        path = path;
    }
}
BENCHMARK_REGISTER_F(GraphFixture, PQ_Vec)
    ->Unit(benchmark::kMillisecond)
    ->Args({0, 4})
    ->Args({1, 40})
    ->Args({2, 400})
    ->Args({3, 4000})
    ->Args({4, 40000})
    ->Args({5, 400000});

BENCHMARK_DEFINE_F(GraphFixture, SortedVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<weight_t> min_distance;
        std::vector<vertex_t> previous;
        DijkstraComputePaths<Container<weight_vertex_pair_t, std::vector, Sorted<weight_vertex_pair_t, std::greater<weight_vertex_pair_t>>>>(0, adjList, min_distance, previous);
        std::list<vertex_t> path = DijkstraGetShortestPathTo(state.range(1), previous);
        path = path;
    }
}
BENCHMARK_REGISTER_F(GraphFixture, SortedVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({0, 4})
    ->Args({1, 40})
    ->Args({2, 400})
    ->Args({3, 4000})
    ->Args({4, 40000})
    ->Args({5, 400000});

BENCHMARK_DEFINE_F(GraphFixture, UnsortedVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::vector<weight_t> min_distance;
        std::vector<vertex_t> previous;
        DijkstraComputePaths<Container<weight_vertex_pair_t, std::vector>>(0, adjList, min_distance, previous);
        std::list<vertex_t> path = DijkstraGetShortestPathTo(state.range(1), previous);
        path = path;
    }
}
BENCHMARK_REGISTER_F(GraphFixture, UnsortedVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({0, 4})
    ->Args({1, 40})
    ->Args({2, 400})
    ->Args({3, 4000})
    ->Args({4, 40000})
    ->Args({5, 400000});

BENCHMARK_MAIN();
