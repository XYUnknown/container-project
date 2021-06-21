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

examples: examples.cpp containers.hpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/examples examples.cpp

benchlookup: benchmark_lookup.cc containers.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_lookup
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_lookup_o2
	$(CC) $(EXT) benchmark_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_lookup_o3

benchinsertion: benchmark_insertion.cc containers.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_insertion
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_insertion_o2
	$(CC) $(EXT) benchmark_insertion.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_insertion_o3

benchinslook: benchmark_insertion_lookup.cc containers.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_insertion_lookup
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_insertion_lookup_o2
	$(CC) $(EXT) benchmark_insertion_lookup.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_insertion_lookup_o3

benchunique: benchmark_unique.cc containers.hpp rng.hpp
	mkdir -p bench
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -o ./bench/benchmark_unique
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./bench/benchmark_unique_o2
	$(CC) $(EXT) benchmark_unique.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./bench/benchmark_unique_o3

oldsource: 
	+$(MAKE) -C old

alltests:
	+$(MAKE) -C tests

clean:
	rm -f *.o
	rm -rf ./old/build
	rm -rf ./tests/build
	rm -rf ./build
	rm -rf ./bench

cleantests:
	rm -rf ./tests/build

cleanbench:
	rm -f ./bench
