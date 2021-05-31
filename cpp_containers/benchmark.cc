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

template<class... Ps>
void Generate(std::size_t amount, std::size_t max_val,
    Container<std::size_t, std::vector, Ps...>& result1,
    Container<std::size_t, std::multiset, Ps...>& result2)
{
    auto v_tmp = Generate(amount, max_val);
    for (auto it=v_tmp.begin(); it!=v_tmp.end(); it++) {
        result1.insert(*it);
        result2.insert(*it);
    }
}
/** Lookup benchmarks */
class SortedLookUpSmallFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 1024; // 8KB data

    std::size_t lookup_size1 = 1000;
    std::size_t lookup_size2 = 10000;
    std::vector<std::size_t> lookups1;
    std::vector<std::size_t> lookups2;

    Container<std::size_t, std::vector, Sorted> v;
    Container<std::size_t, std::multiset, Sorted> s;

    void SetUp(const ::benchmark::State& st) {
        lookups1 = Generate(lookup_size1, size*2);
        lookups2 = Generate(lookup_size2, size*2);
        Generate(size, size*2, v, s);
    }

    void TearDown(const ::benchmark::State&) {
        lookups1.clear();
        lookups2.clear();
        v.clear();
        s.clear();
    }
};

class SortedLookUpMediumFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 10*1024; // 80KB data

    std::size_t lookup_size1 = 1000;
    std::size_t lookup_size2 = 10000;
    std::vector<std::size_t> lookups1;
    std::vector<std::size_t> lookups2;

    Container<std::size_t, std::vector, Sorted> v;
    Container<std::size_t, std::multiset, Sorted> s;

    void SetUp(const ::benchmark::State& st) {
        lookups1 = Generate(lookup_size1, size*2);
        lookups2 = Generate(lookup_size2, size*2);
        Generate(size, size*2, v, s);
    }

    void TearDown(const ::benchmark::State&) {
        lookups1.clear();
        lookups2.clear();
        v.clear();
        s.clear();
    }
};

class SortedLookUpLargeFixture : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 100*1024; // 800KB data

    std::size_t lookup_size1 = 1000;
    std::size_t lookup_size2 = 10000;
    std::vector<std::size_t> lookups1;
    std::vector<std::size_t> lookups2;

    Container<std::size_t, std::vector, Sorted> v;
    Container<std::size_t, std::multiset, Sorted> s;

    void SetUp(const ::benchmark::State& st) {
        lookups1 = Generate(lookup_size1, size*2);
        lookups2 = Generate(lookup_size2, size*2);
        Generate(size, size*2, v, s);
    }

    void TearDown(const ::benchmark::State&) {
        lookups1.clear();
        lookups2.clear();
        v.clear();
        s.clear();
    }
};

BENCHMARK_DEFINE_F(SortedLookUpSmallFixture, SortedVecLookupSmall)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = v.find(item);
            if (it != v.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpSmallFixture, SortedVecLookupSmall);

BENCHMARK_DEFINE_F(SortedLookUpSmallFixture, SortedMultiSetLookupSmall)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = s.find(item);
            if (it != s.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpSmallFixture, SortedMultiSetLookupSmall);

BENCHMARK_DEFINE_F(SortedLookUpMediumFixture, SortedVecLookupMedium)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = v.find(item);
            if (it != v.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpMediumFixture, SortedVecLookupMedium);

BENCHMARK_DEFINE_F(SortedLookUpMediumFixture, SortedMultiSetLookupMedium)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = s.find(item);
            if (it != s.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpMediumFixture, SortedMultiSetLookupMedium);

BENCHMARK_DEFINE_F(SortedLookUpLargeFixture, SortedVecLookupLarge)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = v.find(item);
            if (it != v.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpLargeFixture, SortedVecLookupLarge);

BENCHMARK_DEFINE_F(SortedLookUpLargeFixture, SortedMultiSetLookupLarge)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = s.find(item);
            if (it != s.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedLookUpLargeFixture, SortedMultiSetLookupLarge);

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
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedVectorInsertionSmall);

BENCHMARK_DEFINE_F(InsertionSmallFixture, SortedMultiSetInsertionSmall)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionSmallFixture, SortedMultiSetInsertionSmall);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedVectorInsertionMedium)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedVectorInsertionMedium);

BENCHMARK_DEFINE_F(InsertionMediumFixture, SortedMultiSetInsertionMedium)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionMediumFixture, SortedMultiSetInsertionMedium);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedVectorInsertionLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedVectorInsertionLarge);

BENCHMARK_DEFINE_F(InsertionLargeFixture, SortedMultiSetInsertionLarge)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionLargeFixture, SortedMultiSetInsertionLarge);


BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedVectorInsertionXLarge)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> v;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            v.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedVectorInsertionXLarge);

BENCHMARK_DEFINE_F(InsertionXLargeFixture, SortedMultiSetInsertionXLarge)(benchmark::State& state) {
    Container<std::size_t, std::multiset> s;
    while (state.KeepRunning()) {
        for (std::size_t item : data) {
            s.insert(item);
        }
    }
}
BENCHMARK_REGISTER_F(InsertionXLargeFixture, SortedMultiSetInsertionXLarge);

BENCHMARK_MAIN();