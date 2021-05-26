all: containers.cpp containers_v2.cpp containers_v3.cpp containers_v4.cpp containers_v5.cpp containers_v6.hpp tests.cpp benchmarks.cpp vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers containers.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/vectors vectors.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v2 containers_v2.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v3 containers_v3.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v4 containers_v4.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v5 containers_v5.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/tests tests.cpp
	clang++ -Wall -Werror -std=c++20 -o ./build/benchmarks benchmarks.cpp

containers: containers.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers containers.cpp

containers2: containers_v2.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v2 containers_v2.cpp

containers3: containers_v3.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v3 containers_v3.cpp

containers4: containers_v4.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v4 containers_v4.cpp

containers5: containers_v5.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/containers_v5 containers_v5.cpp

tests: tests.cpp containers_v6.hpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/tests tests.cpp

benchmarks: benchmarks.cpp containers_v6.hpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/benchmarks benchmarks.cpp

vectors: vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++20 -o ./build/vectors vectors.cpp

bench: benchmark.cc
	mkdir -p build
	clang++ benchmark.cc -std=c++20 -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark

clean:
	rm -f *.o ./build/containers *~