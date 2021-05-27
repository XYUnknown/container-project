#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <vector>
#include <set>

#include "containers_v6.hpp"

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

class SortedConFixture : public ::benchmark::Fixture {
public:
    //constexpr static std::size_t up_to = 150000000;
    std::size_t size0 = 10000;
    //std::size_t size1 = 100000;
    //std::size_t size2 = 1024 * 1024;
    //std::size_t size3 = 10 * 1024 * 1024;

    std::size_t lookup_size1 = 1000;
    std::size_t lookup_size2 = 10000;
    std::vector<std::size_t> lookups1;
    std::vector<std::size_t> lookups2;

    Container<std::size_t, std::vector, Sorted> v1;
    Container<std::size_t, std::multiset, Sorted> s1;

    void SetUp(const ::benchmark::State& st) {
        lookups1 = Generate(lookup_size1, size0*2);
        lookups2 = Generate(lookup_size2, size0*2);
        Generate(size0, size0*2, v1, s1);
    }

    void TearDown(const ::benchmark::State&) {
        lookups1.clear();
        lookups2.clear();
        v1.clear();
        s1.clear();
    }

};

BENCHMARK_DEFINE_F(SortedConFixture, SortedVecLookup)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = v1.find(item);
            if (it != v1.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedConFixture, SortedVecLookup);

BENCHMARK_DEFINE_F(SortedConFixture, SortedMultiSetLookup)(benchmark::State& state) {
    volatile std::size_t result;
    while (state.KeepRunning()) {
        for (std::size_t item : lookups1) {
            auto it = s1.find(item);
            if (it != s1.end()) {
                result = *it;
            }
        }
    }
}
BENCHMARK_REGISTER_F(SortedConFixture, SortedMultiSetLookup);

BENCHMARK_MAIN();

