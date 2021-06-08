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
    ->Args({128*1024, 100000}) // 1MB, 100000 lookup
    ->Args({100*128*1024, 10}) // 100MB, 10 lookup
    ->Args({100*128*1024, 1000}) // 100MB, 1000 lookup
    ->Args({100*128*1024, 100000}) // 100MB, 100000 lookup
    ->Args({128*1024*1024, 10}) // 1GB, 10 lookup
    ->Args({128*1024*1024, 1000}) // 1GB, 1000 lookup
    ->Args({128*1024*1024, 100000}); // 1GB, 100000 lookup

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
    ->Args({128*1024}); // 1MB
    //->Args({100*128*1024}) // 100MB
    //->Args({128*1024*1024}); // 1GB

/** Consecutive insertion and lookup benchmarks
    1 insertion 1 lookup
*/
/*BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVectorInsertionLookUpSmall)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedVectorInsertionLookUpSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedMultiSetInsertionLookUpSmall)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedMultiSetInsertionLookUpSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedVectorInsertionLookUpMedium)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedVectorInsertionLookUpMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedMultiSetInsertionLookUpMedium)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedMultiSetInsertionLookUpMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedVectorInsertionLookUpLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedVectorInsertionLookUpLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedMultiSetInsertionLookUpLarge)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedMultiSetInsertionLookUpLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedVectorInsertionLookUpXLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedVectorInsertionLookUpXLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedMultiSetInsertionLookUpXLarge)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            auto it = c.find(item);
            if (it != c.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedMultiSetInsertionLookUpXLarge)->Unit(benchmark::kMillisecond);
*/
/** Consecutive insertion and lookup benchmarks
    1000 insertion 1 lookup
*/
/*BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVector1000InsertionLookUpSmall)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedVector1000InsertionLookUpSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedMultiSet1000InsertionLookUpSmall)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedMultiSet1000InsertionLookUpSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedVector1000InsertionLookUpMedium)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedVector1000InsertionLookUpMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedMultiSet1000InsertionLookUpMedium)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedMultiSet1000InsertionLookUpMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedVector1000InsertionLookUpLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedVector1000InsertionLookUpLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedMultiSet1000InsertionLookUpLarge)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedMultiSet1000InsertionLookUpLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedVector1000InsertionLookUpXLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedVector1000InsertionLookUpXLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedMultiSet1000InsertionLookUpXLarge)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    volatile std::size_t result;
    std::size_t counter = 0;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            c.insert(item);
            if (counter % 1000 == 0) {
                auto it = c.find(item);
                if (it != c.end()) {
                    result = *it;
                }
            }
            counter ++;    
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedMultiSet1000InsertionLookUpXLarge)->Unit(benchmark::kMillisecond);
*/

BENCHMARK_MAIN();