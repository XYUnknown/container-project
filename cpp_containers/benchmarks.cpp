// Benchmarks of the containers implementations version 6
#include <algorithm>
#include <cstdint>
#include <random>
#include <chrono>
#include <iostream>
#include <sstream>
#include <vector>
#include <set>
#include <assert.h>

#include "containers_v6.hpp"

class Timer
{
public:
    Timer() : beg{clock::now()}
    { }
    ~Timer() {
        auto diff = clock::now() - beg;
        auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(diff).count();
        std::ostringstream ss;
        ss.imbue(std::locale("en_US.UTF-8"));
        ss << "Time elapsed: " << ms << " ms";
        std::cout << ss.str() << std::endl;
    }
private:
    using clock = std::chrono::high_resolution_clock;
    clock::time_point beg;
};

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

constexpr std::size_t up_to = 150000000;

int main() {
    std::size_t size1 = 100000;
    //std::size_t size2 = 1024 * 1024;
    //std::size_t size3 = 10 * 1024 * 1024;

    std::size_t lookup_size1 = 1000;
    std::size_t lookup_size2 = 10000;
    std::vector lookups1 = Generate(lookup_size1, up_to);
    std::vector lookups2 = Generate(lookup_size2, up_to);
    volatile std::size_t result; // prevent from optimizing out test code
    
    Container<std::size_t, std::vector, Sorted> v1;
    Container<std::size_t, std::multiset, Sorted> s1;
    Generate(size1, up_to, v1, s1);
    {
        Timer t;
        for (std::size_t item : lookups1) {
            auto it = v1.find(item);
            if (it != v1.end()) {
                result = *it;
            }
        }
    }
    {
        Timer t;
        for (std::size_t item : lookups1) {
            auto it = s1.find(item);
            if (it != s1.end()) {
                result = *it;
            }
        }
    }
    {
        Timer t;
        for (std::size_t item : lookups2) {
            auto it = v1.find(item);
            if (it != v1.end()) {
                result = *it;
            }
        }
    }
    {
        Timer t;
        for (std::size_t item : lookups2) {
            auto it = s1.find(item);
            if (it != s1.end()) {
                result = *it;
            }
        }
    }

    return 0;
}