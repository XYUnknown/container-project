CC = clang++ -std=c++20

OS := $(shell uname)
ifeq ($(OS), Darwin)
	CC = clang++ -std=c++20
else
	CC = g++ -std=c++2a
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

bench: benchmark.cc containers_v6.hpp
	mkdir -p build
	$(CC) benchmark.cc -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark
	$(CC) benchmark.cc -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_o2
	$(CC) benchmark.cc -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_o3

clean:
	rm -f *.o ./build/containers *~