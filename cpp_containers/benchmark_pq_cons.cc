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
    std::size_t step;
    bool r;
    void SetUp(const ::benchmark::State& st) {
        step = st.range(1);
        r = st.range(2);
        data = generate_pairs(5, st.range(0)); // length of each string is 5
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

BENCHMARK_DEFINE_F(PQFixture, Vec_PQ)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        std::size_t counter = 0;
        if (r) {
            bool double_pop = false;
            for (std::pair<std::size_t, std::string> e: data) {
                q.push(e);
                if (counter % step == 0) {
                    if (double_pop) {
                        results.push_back(q.top());
                        q.pop();
                        results.push_back(q.top());
                        q.pop();
                        double_pop = false;
                    } else {
                        results.push_back(q.top());
                        q.pop();
                        double_pop = true;
                    }
                }
                counter ++;
            }
        } else {
            for (std::pair<std::size_t, std::string> e: data) {
                q.push(e);
                if (counter % step == 0) {
                    results.push_back(q.top());
                    q.pop();
                }
                counter ++;
            }
        }
        results = results;
    }
}
BENCHMARK_REGISTER_F(PQFixture, Vec_PQ)
    ->Unit(benchmark::kMillisecond)
    ->Args({32, 2, true}) // 1KB 75% pop
    ->Args({32, 2, false}) // 1KB 50% pop
    ->Args({32, 4, false}) // 1KB 25% pop
    ->Args({10*32, 2, true}) // 10KB
    ->Args({10*32, 2, false}) // 10KB
    ->Args({10*32, 4, false}) // 10KB
    ->Args({100*32, 2, true}) // 100KB
    ->Args({100*32, 2, false}) // 100KB
    ->Args({100*32, 4, false}) // 100KB
    ->Args({32*1024, 2, true}) // 1MB
    ->Args({32*1024, 2, false}) // 1MB
    ->Args({32*1024, 4, false}) // 1MB
    ->Args({10*32*1024, 2, true}) // 10MB
    ->Args({10*32*1024, 2, false}) // 10MB
    ->Args({10*32*1024, 4, false}) // 10MB
    ->Args({100*32*1024, 2, true}) // 100MB
    ->Args({100*32*1024, 2, false}) // 100MB
    ->Args({100*32*1024, 4, false}) // 100MB
    ->Args({32*1024*1024, 2, true}) // 1GB
    ->Args({32*1024*1024, 2, false}) // 1GB
    ->Args({32*1024*1024, 4, false}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Deque_PQ)(benchmark::State& state) {
    while (state.KeepRunning()) {
        std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        std::size_t counter = 0;
        if (r) {
            bool double_pop = false;
            for (std::pair<std::size_t, std::string> e: data) {
                q.push(e);
                if (counter % step == 0) {
                    if (double_pop) {
                        results.push_back(q.top());
                        q.pop();
                        results.push_back(q.top());
                        q.pop();
                        double_pop = false;
                    } else {
                        results.push_back(q.top());
                        q.pop();
                        double_pop = true;
                    }
                }
                counter ++;
            }
        } else {
            for (std::pair<std::size_t, std::string> e: data) {
                q.push(e);
                if (counter % step == 0) {
                    results.push_back(q.top());
                    q.pop();
                }
                counter ++;
            }
        }
        results = results;
    }
}
BENCHMARK_REGISTER_F(PQFixture, Deque_PQ)
    ->Unit(benchmark::kMillisecond)
    ->Args({32, 2, true}) // 1KB 75% pop
    ->Args({32, 2, false}) // 1KB 50% pop
    ->Args({32, 4, false}) // 1KB 25% pop
    ->Args({10*32, 2, true}) // 10KB
    ->Args({10*32, 2, false}) // 10KB
    ->Args({10*32, 4, false}) // 10KB
    ->Args({100*32, 2, true}) // 100KB
    ->Args({100*32, 2, false}) // 100KB
    ->Args({100*32, 4, false}) // 100KB
    ->Args({32*1024, 2, true}) // 1MB
    ->Args({32*1024, 2, false}) // 1MB
    ->Args({32*1024, 4, false}) // 1MB
    ->Args({10*32*1024, 2, true}) // 10MB
    ->Args({10*32*1024, 2, false}) // 10MB
    ->Args({10*32*1024, 4, false}) // 10MB
    ->Args({100*32*1024, 2, true}) // 100MB
    ->Args({100*32*1024, 2, false}) // 100MB
    ->Args({100*32*1024, 4, false}) // 100MB
    ->Args({32*1024*1024, 2, true}) // 1GB
    ->Args({32*1024*1024, 2, false}) // 1GB
    ->Args({32*1024*1024, 4, false}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, SortedOnAccessVec)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::vector, SortedOnAccess<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        std::size_t counter = 0;
        if (r) {
            bool double_pop = false;
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    if (double_pop) {
                        results.push_back(q.back());
                        q.pop_back();
                        results.push_back(q.back());
                        q.pop_back();
                        double_pop = false;
                    } else {
                        results.push_back(q.back());
                        q.pop_back();
                        double_pop = true;
                    }
                }
                counter ++;
            }
        } else {
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    results.push_back(q.back());
                    q.pop_back();
                }
                counter ++;
            }
        }
        results = results;
    }
}
BENCHMARK_REGISTER_F(PQFixture, SortedOnAccessVec)
    ->Unit(benchmark::kMillisecond)
    ->Args({32, 2, true}) // 1KB 75% pop
    ->Args({32, 2, false}) // 1KB 50% pop
    ->Args({32, 4, false}) // 1KB 25% pop
    ->Args({10*32, 2, true}) // 10KB
    ->Args({10*32, 2, false}) // 10KB
    ->Args({10*32, 4, false}) // 10KB
    ->Args({100*32, 2, true}) // 100KB
    ->Args({100*32, 2, false}) // 100KB
    ->Args({100*32, 4, false}) // 100KB
    ->Args({32*1024, 2, true}) // 1MB
    ->Args({32*1024, 2, false}) // 1MB
    ->Args({32*1024, 4, false}) // 1MB
    ->Args({10*32*1024, 2, true}) // 10MB
    ->Args({10*32*1024, 2, false}) // 10MB
    ->Args({10*32*1024, 4, false}); // 10MB
    //->Args({100*32*1024, 2, true}) // 100MB
    //->Args({100*32*1024, 2, false}) // 100MB
    //->Args({100*32*1024, 4, false}); // 100MB
    //->Args({32*1024*1024, 2, true}) // 1GB
    //->Args({32*1024*1024, 2, false}); // 1GB
    //->Args({32*1024*1024, 4, false}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, Tree)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, TreeWrapper> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        std::size_t counter = 0;
        if (r) {
            bool double_pop = false;
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    if (double_pop) {
                        results.push_back(*q.rbegin());
                        q.erase(--q.end());
                        results.push_back(*q.rbegin());
                        q.erase(--q.end());
                        double_pop = false;
                    } else {
                        results.push_back(*q.rbegin());
                        q.erase(--q.end());
                        double_pop = true;
                    }
                }
                counter ++;
            }
        } else {
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    results.push_back(*q.rbegin());
                    q.erase(--q.end());
                }
                counter ++;
            }
        }
        results = results;
    }
}
BENCHMARK_REGISTER_F(PQFixture, Tree)
    ->Unit(benchmark::kMillisecond)
    ->Args({32, 2, true}) // 1KB 75% pop
    ->Args({32, 2, false}) // 1KB 50% pop
    ->Args({32, 4, false}) // 1KB 25% pop
    ->Args({10*32, 2, true}) // 10KB
    ->Args({10*32, 2, false}) // 10KB
    ->Args({10*32, 4, false}) // 10KB
    ->Args({100*32, 2, true}) // 100KB
    ->Args({100*32, 2, false}) // 100KB
    ->Args({100*32, 4, false}) // 100KB
    ->Args({32*1024, 2, true}) // 1MB
    ->Args({32*1024, 2, false}) // 1MB
    ->Args({32*1024, 4, false}) // 1MB
    ->Args({10*32*1024, 2, true}) // 10MB
    ->Args({10*32*1024, 2, false}) // 10MB
    ->Args({10*32*1024, 4, false}) // 10MB
    ->Args({100*32*1024, 2, true}) // 100MB
    ->Args({100*32*1024, 2, false}) // 100MB
    ->Args({100*32*1024, 4, false}) // 100MB
    ->Args({32*1024*1024, 2, true}) // 1GB
    ->Args({32*1024*1024, 2, false}) // 1GB
    ->Args({32*1024*1024, 4, false}); // 1GB

BENCHMARK_DEFINE_F(PQFixture, SortedOnAccessList)(benchmark::State& state) {
    while (state.KeepRunning()) {
        Container<std::pair<std::size_t, std::string>, std::list, SortedOnAccess<std::pair<std::size_t, std::string>>> q;
        Container<std::pair<std::size_t, std::string>, std::vector> results;
        std::size_t counter = 0;
        if (r) {
            bool double_pop = false;
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    if (double_pop) {
                        results.push_back(q.back());
                        q.pop_back();
                        results.push_back(q.back());
                        q.pop_back();
                        double_pop = false;
                    } else {
                        results.push_back(q.back());
                        q.pop_back();
                        double_pop = true;
                    }
                }
                counter ++;
            }
        } else {
            for (std::pair<std::size_t, std::string> e: data) {
                q.insert(e);
                if (counter % step == 0) {
                    results.push_back(q.back());
                    q.pop_back();
                }
                counter ++;
            }
        }
        results = results;
    }
}
BENCHMARK_REGISTER_F(PQFixture, SortedOnAccessList)
    ->Unit(benchmark::kMillisecond)
    ->Args({32, 2, true}) // 1KB 75% pop
    ->Args({32, 2, false}) // 1KB 50% pop
    ->Args({32, 4, false}) // 1KB 25% pop
    ->Args({10*32, 2, true}) // 10KB
    ->Args({10*32, 2, false}) // 10KB
    ->Args({10*32, 4, false}) // 10KB
    ->Args({100*32, 2, true}) // 100KB
    ->Args({100*32, 2, false}) // 100KB
    ->Args({100*32, 4, false}) // 100KB
    ->Args({32*1024, 2, true}) // 1MB
    ->Args({32*1024, 2, false}) // 1MB
    ->Args({32*1024, 4, false}); // 1MB
    //->Args({10*32*1024, 2, true}) // 10MB
    //->Args({10*32*1024, 2, false}) // 10MB
    //->Args({10*32*1024, 4, false}) // 10MB
    //->Args({100*32*1024, 2, true}) // 100MB
    //->Args({100*32*1024, 2, false}) // 100MB
    //->Args({100*32*1024, 4, false}) // 100MB
    //->Args({32*1024*1024, 2, true}) // 1GB
    //->Args({32*1024*1024, 2, false}) // 1GB
    //->Args({32*1024*1024, 4, false}); // 1GB

BENCHMARK_MAIN();