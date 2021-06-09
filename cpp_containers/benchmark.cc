#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers_v6.hpp"

/** Data generation */
std::vector<std::size_t> Generate(std::size_t size) {
    std::minstd_rand generator;
    generator.seed(size);
    std::vector<std::size_t> v(size);
    std::generate(v.begin(), v.end(), generator);
    return v;
}

template<class T, template<typename...> class C, class ...Ps>
void Generate(Container<T, C, Ps...>& c, std::size_t size) {
    std::minstd_rand generator;
    generator.seed(size);
    for (std::size_t i=0; i < size; i++)
        c.insert((std::size_t)generator());
}

template<class P, class ...Ps>
constexpr bool has_property()
{ return std::disjunction_v<std::is_same<P, Ps>...>; }

/** Lookup benchmarks */
template<template<typename...> class C, class ...Ps>
class LookUpFixture: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    //std::size_t size = 128; // 1KB data
    Container<std::size_t, C, Ps...> c;

    //std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(st.range(1));
        Generate(c, st.range(0));
        if constexpr (has_property<SortedOnAccess, Ps...>()) {
            c.sort(); // for sorted vectors and sorted lists
        }
    }

    void TearDown(const ::benchmark::State&) {
        lookups.clear();
        c.clear();
    }
};

// Unsorted Vector Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, UnsortedVec_LookUp, std::vector)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, UnsortedVec_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}); // 1MB, 100000 lookup
    //->Args({100*128*1024, 10}) // 100MB, 10 lookup
    //->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Sorted Vector Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, SortedOnAccessVec_LookUp, std::vector, SortedOnAccess)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, SortedOnAccessVec_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({100*128*1024, 10}) // 100MB, 10 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Binary Search Tree (Multiset) Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, BST_LookUp, TreeWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, BST_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({100*128*1024, 10}) // 100MB, 10 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// TreeSet Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, TreeSet_LookUp, TreeSetWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, TreeSet_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({100*128*1024, 10}) // 100MB, 10 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// HashSet Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, HashSet_LookUp, HashSetWrapper)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, HashSet_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({100*128*1024, 10}) // 100MB, 10 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Sorted List Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, SortedList_LookUp, std::list, SortedOnAccess)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, SortedList_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}); // 1MB, 100000 lookup
    //->Args({100*128*1024, 10}) // 100MB, 10 lookup
    //->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

// Unsorted List Lookup
BENCHMARK_TEMPLATE_DEFINE_F(LookUpFixture, UnsortedList_LookUp, std::list)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups) {
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(LookUpFixture, UnsortedList_LookUp)
    ->Unit(benchmark::kMillisecond)
    ->Args({128, 10}) // 1KB, 10 lookup
    ->Args({128, 1000}) // 1KB, 1000 lookup
    ->Args({128, 100000}) // 1KB, 100000 lookup
    ->Args({100*128, 10}) // 100KB, 10 lookup
    ->Args({100*128, 1000}) // 100KB, 1000 lookup
    ->Args({100*128, 100000}) // 100KB, 100000 lookup
    ->Args({128*1024, 10}) // 1MB, 10 lookup
    ->Args({128*1024, 1000}) // 1MB, 1000 lookup
    ->Args({128*1024, 100000}); // 1MB, 100000 lookup
    //->Args({100*128*1024, 10}) // 100MB, 10 lookup
    //->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    //->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    //->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    //->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

/** Insertion benchmarks */
class InsertionFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size;  // 1KB data
    std::minstd_rand generator;

    void SetUp(const ::benchmark::State& st) {
        size = st.range(0);
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

/* BST (multiset) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, BST_Insertion)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, BST_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/* Sorted vector insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedVector_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedVector_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}); // 1MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

/* Sorted vector (on access) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessVector_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessVector_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/*  tree set insertion */
BENCHMARK_DEFINE_F(InsertionFixture, TreeSet_Insertion)(benchmark::State& state) {
    Container<std::size_t, TreeSetWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, TreeSet_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/*  hash set insertion */
BENCHMARK_DEFINE_F(InsertionFixture, HashSet_Insertion)(benchmark::State& state) {
    Container<std::size_t, HashSetWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, HashSet_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

/* Sorted list insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedList_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::list, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedList_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}); // 1MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

/* Sorted list (on access) insertion */
BENCHMARK_DEFINE_F(InsertionFixture, SortedOnAccessList_Insertion)(benchmark::State& state) {
    Container<std::size_t, std::list, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture, SortedOnAccessList_Insertion)
    ->Unit(benchmark::kMillisecond)
    ->Args({128}) // 1KB
    ->Args({100*128}) // 100KB
    ->Args({128*1024}) // 1MB
    ->Args({100*128*1024}) // 100MB
    ->Args({128*1024*1024}); // 1GB

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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step

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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
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
    ->Args({100*128, 1}) // 100KB, 1 step
    ->Args({100*128, 10}) // 100KB, 10 step
    ->Args({100*128, 100}) // 100KB, 100 step
    ->Args({128*1024, 1}) // 1MB, 1 step
    ->Args({128*1024, 100}) // 1MB, 100 step
    ->Args({128*1024, 10000}); // 1MB, 10000 step
    //->Args({100*128*1024, 1}) // 100MB, 1 step
    //->Args({100*128*1024, 100}) // 100MB, 100 step
    //->Args({100*128*1024, 10000}); // 100MB, 10000 step
    //->Args({128*1024*1024, 1}) // 1GB, 1 step
    //->Args({128*1024*1024, 1000}) // 1GB, 1000 step
    //->Args({128*1024*1024, 1000000}); // 1GB, 1000000 step


BENCHMARK_MAIN();