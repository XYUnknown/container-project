CC = clang++ -std=c++20
CMP = /afs/inf.ed.ac.uk/user/s16/s1673055/tools/clang+llvm-12.0.0-x86_64-linux-sles12.4
INCLUDE = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/include/benchmark
LIB = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/lib
CON = /Users/yukiko/Documents/container-project/cpp_containers/includes

EXT =

OS := $(shell uname)
ifeq ($(OS), Darwin)
	CC = clang++ -std=c++20 -I$(CON)
else
	CC = $(CMP)/bin/clang++ -std=c++20 -stdlib=libc++ -nostdinc++ -I$(CMP)/include/c++/v1 -L$(CMP)/lib -Wl,-rpath,$(CMP)/lib
	EXT = -I$(INCLUDE) -L$(LIB)
endif

examples: examples.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/examples examples.cpp

dijkstra: dijkstra_examples.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/dijkstra dijkstra_examples.cpp

prims: prims_examples.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/prims prims_examples.cpp

oldsource: 
	+$(MAKE) -C old

alltests:
	+$(MAKE) -C tests

allbench:
	+$(MAKE) -C bench

clean:
	rm -f *.o
	rm -rf ./old/build
	rm -rf ./tests/build
	rm -rf ./bench/build
	rm -rf ./build

cleantests:
	rm -rf ./tests/build

cleanbench:
	rm -rf ./bench/build
