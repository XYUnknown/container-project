#include <benchmark/benchmark.h>
#include <algorithm>

#include "powerset.hpp"
#include "rng.hpp"

template<class P, class ...Ps>
constexpr bool has_property()
{ return std::disjunction_v<std::is_same<P, Ps>...>; }

template<class C>
class PowersetFixture: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    C set;
    std::minstd_rand generator;
    std::size_t size;

    void SetUp(const ::benchmark::State& st) {
        size = st.range(0);
        generator.seed(size);
        while (set.size() < size) {
            set.insert((std::size_t)generator());
        }

        if constexpr (std::is_same<C, Container<std::size_t, std::vector, Iterable>>::value) {
            std::sort(set.begin(), set.end());
            set.erase(std::unique(set.begin(), set.end()), set.end());
        }
        
    }
    void TearDown(const ::benchmark::State&) {
        set.clear();
    }
};

BENCHMARK_TEMPLATE_DEFINE_F(PowersetFixture, OrderedSet, Container<std::size_t, TreeSetWrapperAsc, Iterable>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        auto pset = powerset<Container<std::size_t, TreeSetWrapperAsc, Iterable>, TreeSetWrapperAsc, Iterable>(set);
        //cstd::cout << "Number of powersets: " << pset.size() << std::endl;
    }
}
BENCHMARK_REGISTER_F(PowersetFixture, OrderedSet)
    ->Unit(benchmark::kMillisecond)
    ->Args({5})
    ->Args({10})
    ->Args({15})
    ->Args({20});

BENCHMARK_TEMPLATE_DEFINE_F(PowersetFixture, UnorderedSet, Container<std::size_t, HashSetWrapper, Iterable>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        auto pset = powerset<Container<std::size_t, HashSetWrapper, Iterable>, HashSetWrapperH, Iterable>(set);
    }
}
BENCHMARK_REGISTER_F(PowersetFixture, UnorderedSet)
    ->Unit(benchmark::kMillisecond)
    ->Args({5})
    ->Args({10})
    ->Args({15})
    ->Args({20});

BENCHMARK_TEMPLATE_DEFINE_F(PowersetFixture, Vector, Container<std::size_t, std::vector, Iterable>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        auto pset = powerset<Container<std::size_t, std::vector, Iterable>, std::vector, Iterable>(set);
    }
}
BENCHMARK_REGISTER_F(PowersetFixture, Vector)
    ->Unit(benchmark::kMillisecond)
    ->Args({5})
    ->Args({10})
    ->Args({15})
    ->Args({20});

BENCHMARK_TEMPLATE_DEFINE_F(PowersetFixture, UniqueVectorLazy, Container<std::size_t, std::vector, Unique<false>, LookUp, Iterable>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        auto pset = powerset<Container<std::size_t, std::vector, Unique<false>, LookUp, Iterable>, std::vector, Unique<false>, LookUp, Iterable>(set);
        pset.size();
    }
}
BENCHMARK_REGISTER_F(PowersetFixture, UniqueVectorLazy)
    ->Unit(benchmark::kMillisecond)
    ->Args({5})
    ->Args({10})
    ->Args({15})
    ->Args({20});

BENCHMARK_TEMPLATE_DEFINE_F(PowersetFixture, UniqueVectorEager, Container<std::size_t, std::vector, Unique<>, LookUp, Iterable>)(benchmark::State& state) {
    while (state.KeepRunning()) {
        auto pset = powerset<Container<std::size_t, std::vector, Unique<>, LookUp, Iterable>, std::vector, Unique<>, LookUp, Iterable>(set);
    }
}
BENCHMARK_REGISTER_F(PowersetFixture, UniqueVectorEager)
    ->Unit(benchmark::kMillisecond)
    ->Args({5})
    ->Args({10})
    ->Args({15})
    ->Args({20});

BENCHMARK_MAIN();