#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers_v6.hpp"

/** Data generation */
template<typename RndEngine>
struct DataGen
{
    RndEngine e;
    const std::size_t max_val;
    DataGen(std::size_t max_val_) : max_val{max_val_} { }
    std::size_t operator()() {
        std::size_t ret = e();
        return ret % (max_val + 1);
    }
};

std::vector<std::size_t> Generate(std::size_t amount, std::size_t max_val)
{
    DataGen<std::minstd_rand> generator{max_val};
    std::vector<std::size_t> v(amount);
    std::generate(std::begin(v), std::end(v), generator);
    return v;
}

template<class T, template<typename...> class C, class ...Ps>
void Copy(std::vector<T> data, Container<T, C, Ps...>& result)
{
    for (auto it=data.begin(); it!=data.end(); it++) {
        result.insert(*it);
    }
}

/** Lookup benchmarks */
template<template<typename...> class C>
class SortedLookUpSmallFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 1024; // 8KB data
    std::vector<std::size_t> data;

    std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    Container<std::size_t, C, Sorted> c;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size, size*2);
        data = Generate(size, size*2);
        Copy(data, c);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
        lookups.clear();
        c.clear();
    }
};

template<template<typename...> class C>
class SortedLookUpMediumFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 10*1024; // 80KB data
    std::vector<std::size_t> data;

    std::size_t lookup_size = 1000;
    std::vector<std::size_t> lookups;

    Container<std::size_t, C, Sorted> c;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size, size*2);
        data = Generate(size, size*2);
        Copy(data, c);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
        lookups.clear();
        c.clear();
    }
};

template<template<typename...> class C>
class SortedLookUpLargeFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 100*1024; // 800KB data
    std::vector<std::size_t> data;

    std::size_t lookup_size = 1000;
    std::vector<std::size_t> lookups;

    Container<std::size_t, C, Sorted> c;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size, size*2);
        data = Generate(size, size*2);
        Copy(data, c);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
        lookups.clear();
        c.clear();
    }
};

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpSmallFixture, SortedVecLookupSmall, std::vector)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpSmallFixture, SortedVecLookupSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpSmallFixture, SortedMultiSetLookupSmall, std::multiset)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpSmallFixture, SortedMultiSetLookupSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpMediumFixture, SortedVecLookupMedium, std::vector)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpMediumFixture, SortedVecLookupMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpMediumFixture, SortedMultiSetLookupMedium, std::multiset)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpMediumFixture, SortedMultiSetLookupMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpLargeFixture, SortedVecLookupLarge, std::vector)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpLargeFixture, SortedVecLookupLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(SortedLookUpLargeFixture, SortedMultiSetLookupLarge, std::multiset)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(SortedLookUpLargeFixture, SortedMultiSetLookupLarge)->Unit(benchmark::kMillisecond);

/** Insertion benchmarks */
class InsertionSmallFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 1024; // 8KB data
    std::vector<std::size_t> data;

    void SetUp(const ::benchmark::State& st) {
        data = Generate(size, size);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

class InsertionMediumFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 10*1024; // 80KB data
    std::vector<std::size_t> data;

    void SetUp(const ::benchmark::State& st) {
        data = Generate(size, size);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

class InsertionLargeFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 100*1024; // 800KB data
    std::vector<std::size_t> data;

    void SetUp(const ::benchmark::State& st) {
        data = Generate(size, size);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

class InsertionXLargeFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 1024*1024; // 8MB data
    std::vector<std::size_t> data;

    void SetUp(const ::benchmark::State& st) {
        data = Generate(size, size);
    }

    void TearDown(const ::benchmark::State&) {
        data.clear();
    }
};

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVectorInsertionSmall)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedVectorInsertionSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVectorReservedInsertionSmall)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    v.reserve(size);
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedVectorReservedInsertionSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedMultiSetInsertionSmall)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedMultiSetInsertionSmall)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedVectorInsertionMedium)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedVectorInsertionMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedVectorReservedInsertionMedium)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    v.reserve(size);
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedVectorReservedInsertionMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedMultiSetInsertionMedium)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedMultiSetInsertionMedium)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedVectorInsertionLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedVectorInsertionLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedVectorReservedInsertionLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    v.reserve(size);
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedVectorReservedInsertionLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedMultiSetInsertionLarge)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedMultiSetInsertionLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedVectorInsertionXLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedVectorInsertionXLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedVectorReservedInsertionXLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    v.reserve(size);
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedVectorReservedInsertionXLarge)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedMultiSetInsertionXLarge)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedMultiSetInsertionXLarge)->Unit(benchmark::kMillisecond);

/** Consecutive insertion and lookup benchmarks
    1 insertion 1 lookup
*/
BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVectorInsertionLookUpSmall)(benchmark::State& state) {
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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

/** Consecutive insertion and lookup benchmarks
    1000 insertion 1 lookup
*/
BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedVector1000InsertionLookUpSmall)(benchmark::State& state) {
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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
    Container<std::size_t, std::multiset, Sorted> c;
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


BENCHMARK_MAIN();