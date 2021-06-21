#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers.hpp"
#include "rng.hpp"

/* Same fixture as insertion */
class InsertionFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size;
    std::minstd_rand generator;

    void SetUp(const ::benchmark::State& st) {
        size = st.range(0);
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};
/** Consecutive insertion and lookup benchmarks */
/* BST (multiset) consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, BST_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, BST_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}) // 1MB, 10000 step
    ->Args({10*128*1024, 1}) // 10MB, 1 step
    ->Args({10*128*1024, 100}) // 10MB, 100 step
    ->Args({10*128*1024, 10000}); // 10MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Sorted vector consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, SortedVector_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedVector_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}) // 1MB, 10000 step
    ->Args({10*128*1024, 1}) // 10MB, 1 step
    ->Args({10*128*1024, 100}) // 10MB, 100 step
    ->Args({10*128*1024, 10000}); // 10MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Sorted vector (on access) consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessVector_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessVector_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    //->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}) // 1MB, 10000 step
    //->Args({10*128*1024, 1}) // 10MB, 1 step
    ->Args({10*128*1024, 100}) // 10MB, 100 step
    ->Args({10*128*1024, 10000}); // 10MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Tree set consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, TreeSet_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, TreeSetWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, TreeSet_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}) // 1MB, 10000 step
    ->Args({10*128*1024, 1}) // 10MB, 1 step
    ->Args({10*128*1024, 100}) // 10MB, 100 step
    ->Args({10*128*1024, 10000}) // 10MB, 10000 step
    ->Args({100*128*1024, 1}) // 100MB, 1 step
    ->Args({100*128*1024, 100}) // 100MB, 100 step
    ->Args({100*128*1024, 10000}) // 100MB, 10000 step
    ->Args({128*1024*1024, 1}) // 1GB, 1 step
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    ->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Hash set consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, HashSet_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, HashSetWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, HashSet_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}) // 1MB, 10000 step
    ->Args({10*128*1024, 1}) // 10MB, 1 step
    ->Args({10*128*1024, 100}) // 10MB, 100 step
    ->Args({10*128*1024, 10000}) // 10MB, 10000 step
    ->Args({100*128*1024, 1}) // 100MB, 1 step
    ->Args({100*128*1024, 100}) // 100MB, 100 step
    ->Args({100*128*1024, 10000}) // 100MB, 10000 step
    ->Args({128*1024*1024, 1}) // 1GB, 1 step
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    ->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Sorted list consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, SortedList_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, std::list, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedList_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
    //->Args({10*128*1024, 1}) // 10MB, 1 step
    //->Args({10*128*1024, 100}) // 10MB, 100 step
    //->Args({10*128*1024, 10000}); // 10MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

/* Sorted list (on access) consecutive insertion and lookup */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessList_Insertion_LookUp)(benchmark::State& state) {
    Container<std::size_t, std::list, SortedOnAccess> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    std::size_t step = state.range(1);
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++) {
            c.insert((std::size_t)generator());
            if (counter % step == 0) {
                auto it = c.find((std::size_t)generator());
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++; 
        }
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessList_Insertion_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 1}) // 1KB, 1 step
    ->Args({128, 10}) // 1KB, 10 step
    ->Args({128, 100}) // 1KB, 100 step
    ->Args({10*128, 1}) // 10KB, 1 step
    ->Args({10*128, 10}) // 10KB, 10 step
    ->Args({10*128, 100}) // 10KB, 100 step
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
    //->Args({10*128*1024, 1}) // 10MB, 1 step
    //->Args({10*128*1024, 100}) // 10MB, 100 step
    //->Args({10*128*1024, 10000}); // 10MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

BENCHMARK_MAIN();