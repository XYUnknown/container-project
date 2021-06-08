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
    std::mt19937 generator;
    generator.seed(size);
    std::vector<std::size_t> v(size);
    std::generate(v.begin(), v.end(), generator);
    return v;
}

template<class T, template<typename...> class C, class ...Ps>
void Generate(Container<T, C, Ps...>& c, std::size_t size) {
    std::mt19937 generator;
    generator.seed(size);
    for (std::size_t i=0; i < size; i++)
        c.insert((std::size_t)generator());
}

/** Lookup benchmarks */
template<template<typename...> class C, class ...Ps>
class LookUp_1000_1KB: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 128; // 1KB data
    Container<std::size_t, C, Ps...> c;

    std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size);
        Generate(c, size);
    }

    void TearDown(const ::benchmark::State&) {
        lookups.clear();
        c.clear();
    }
};

template<template<typename...> class C, class ...Ps>
class LookUp_1000_100KB: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 25*512; // 100KB data
    Container<std::size_t, C, Ps...> c;

    std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size);
        Generate(c, size);
    }

    void TearDown(const ::benchmark::State&) {
        lookups.clear();
        c.clear();
    }
};

template<template<typename...> class C, class ...Ps>
class LookUp_1000_1MB: public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 128*1024; // 1MB data
    Container<std::size_t, C, Ps...> c;

    std::size_t lookup_size = 1000; // perform 1000 lookups
    std::vector<std::size_t> lookups;

    void SetUp(const ::benchmark::State& st) {
        lookups = Generate(lookup_size);
        Generate(c, size);
    }

    void TearDown(const ::benchmark::State&) {
        lookups.clear();
        c.clear();
    }
};

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, SortedVec_1KB_1000LookUp, std::vector, Sorted)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, SortedVec_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, SortedOnAccessVec_1KB_1000LookUp, std::vector, SortedOnAccess)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, SortedOnAccessVec_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, BST_1KB_1000LookUp, TreeWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, BST_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, HashSet_1KB_1000LookUp, HashSetWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, HashSet_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, TreeSet_1KB_1000LookUp, TreeSetWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, TreeSet_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, SortedList_1KB_1000LookUp, std::list, Sorted)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, SortedList_1KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_1KB, SortedOnAccessList_1KB_1000LookUp, std::list, SortedOnAccess)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_1KB, SortedOnAccessList_1KB_1000LookUp)->Unit(benchmark::kMillisecond);


BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, SortedVec_100KB_1000LookUp, std::vector, Sorted)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, SortedVec_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, SortedOnAccessVec_100KB_1000LookUp, std::vector, SortedOnAccess)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, SortedOnAccessVec_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, BST_100KB_1000LookUp, TreeWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, BST_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, HashSet_100KB_1000LookUp, HashSetWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, HashSet_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, TreeSet_100KB_1000LookUp, TreeSetWrapper)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, TreeSet_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, SortedList_100KB_1000LookUp, std::list, Sorted)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, SortedList_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

BENCHMARK_TEMPLATE_DEFINE_F(LookUp_1000_100KB, SortedOnAccessList_100KB_1000LookUp, std::list, SortedOnAccess)(benchmark::State& state) {
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
BENCHMARK_REGISTER_F(LookUp_1000_100KB, SortedOnAccessList_100KB_1000LookUp)->Unit(benchmark::kMillisecond);

/** Insertion benchmarks */
class InsertionFixture_1KB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 128; // 1KB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

class InsertionFixture_100KB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 25*512; // 100KB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

class InsertionFixture_1MB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 128*1024; // 1MB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

class InsertionFixture_100MB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 100*128*1024; // 100MB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

class InsertionFixture_1GB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 128*1024*1024; // 1GB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

class InsertionFixture_4GB : public ::benchmark::Fixture {
public:
    // size_t --> 8 bytes
    std::size_t size = 512*1024*1024; // 4GB data
    std::mt19937 generator;

    void SetUp(const ::benchmark::State& st) {
        generator.seed(size);
    }

    void TearDown(const ::benchmark::State&) {
    }
};

BENCHMARK_DEFINE_F(InsertionFixture_1KB, SortedMultiSetInsertion_1KB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1KB, SortedMultiSetInsertion_1KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1KB, SortedVectorInsertion_1KB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1KB, SortedVectorInsertion_1KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1KB, SortedOnAccessVectorInsertion_1KB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1KB, SortedOnAccessVectorInsertion_1KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100KB, SortedMultiSetInsertion_100KB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100KB, SortedMultiSetInsertion_100KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100KB, SortedVectorInsertion_100KB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100KB, SortedVectorInsertion_100KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100KB, SortedOnAccessVectorInsertion_100KB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100KB, SortedOnAccessVectorInsertion_100KB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1MB, SortedMultiSetInsertion_1MB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1MB, SortedMultiSetInsertion_1MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1MB, SortedVectorInsertion_1MB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1MB, SortedVectorInsertion_1MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1MB, SortedOnAccessVectorInsertion_1MB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1MB, SortedOnAccessVectorInsertion_1MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100MB, SortedMultiSetInsertion_100MB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100MB, SortedMultiSetInsertion_100MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100MB, SortedVectorInsertion_100MB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100MB, SortedVectorInsertion_100MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_100MB, SortedOnAccessVectorInsertion_100MB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_100MB, SortedOnAccessVectorInsertion_100MB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1GB, SortedMultiSetInsertion_1GB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1GB, SortedMultiSetInsertion_1GB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1GB, SortedVectorInsertion_1GB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1GB, SortedVectorInsertion_1GB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_1GB, SortedOnAccessVectorInsertion_1GB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_1GB, SortedOnAccessVectorInsertion_1GB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_4GB, SortedMultiSetInsertion_4GB)(benchmark::State& state) {
    Container<std::size_t, TreeWrapper> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_4GB, SortedMultiSetInsertion_4GB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_4GB, SortedVectorInsertion_4GB)(benchmark::State& state) {
    Container<std::size_t, std::vector, Sorted> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_4GB, SortedVectorInsertion_4GB)->Unit(benchmark::kMillisecond);

BENCHMARK_DEFINE_F(InsertionFixture_4GB, SortedOnAccessVectorInsertion_4GB)(benchmark::State& state) {
    Container<std::size_t, std::vector, SortedOnAccess> c;
    while (state.KeepRunning()) {
        for (std::size_t i=0; i < size; i++)
            c.insert((std::size_t)generator());
    }
}
BENCHMARK_REGISTER_F(InsertionFixture_4GB, SortedOnAccessVectorInsertion_4GB)->Unit(benchmark::kMillisecond);

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