CC = clang++ -std=c++20
CMP = /afs/inf.ed.ac.uk/user/s16/s1673055/tools/clang+llvm-12.0.0-x86_64-linux-sles12.4
INCLUDE = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/include/benchmark
LIB = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/lib

EXT =

OS := $(shell uname)
ifeq ($(OS), Darwin)
	CC = clang++ -std=c++20
else
	CC = $(CMP)/bin/clang++ -std=c++20 -stdlib=libc++ -nostdinc++ -I$(CMP)/include/c++/v1 -L$(CMP)/lib -Wl,-rpath,$(CMP)/lib
	EXT = -I$(INCLUDE) -L$(LIB)
endif

all: containers.cpp containers_v2.cpp containers_v3.cpp containers_v4.cpp containers_v5.cpp containers_v6.hpp tests.cpp vectors.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers containers.cpp
	$(CC) -Wall -Werror -o ./build/vectors vectors.cpp
	$(CC) -Wall -Werror -o ./build/containers_v2 containers_v2.cpp
	$(CC) -Wall -Werror -o ./build/containers_v3 containers_v3.cpp
	$(CC) -Wall -Werror -o ./build/containers_v4 containers_v4.cpp
	$(CC) -Wall -Werror -o ./build/containers_v5 containers_v5.cpp
	$(CC) -Wall -Werror -o ./build/tests tests.cpp

containers: containers.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers containers.cpp

containers2: containers_v2.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v2 containers_v2.cpp

containers3: containers_v3.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v3 containers_v3.cpp

containers4: containers_v4.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v4 containers_v4.cpp

containers5: containers_v5.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v5 containers_v5.cpp

tests: tests.cpp containers_v6.hpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/tests tests.cpp

vectors: vectors.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/vectors vectors.cpp

benchlookup: benchmark_lookup.cc containers_v6.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_lookup
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_lookup_o2
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_lookup_o3

benchinsertion: benchmark_insertion.cc containers_v6.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_insertion
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_insertion_o2
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_insertion_o3

benchinslook: benchmark_insertion_lookup.cc containers_v6.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_insertion_lookup
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_insertion_lookup_o2
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_insertion_lookup_o3

benchunique: benchmark_unique.cc containers_v6.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_unique
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_unique_o2
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_unique_o3

clean:
	rm -f *.o
	rm -rf ./build
	rm -rf ./bench

cleanbench:
	rm -f ./bench
