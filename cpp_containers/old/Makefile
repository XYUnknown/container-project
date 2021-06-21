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

all: containers_v1.cpp containers_v2.cpp containers_v3.cpp containers_v4.cpp containers_v5.cpp vectors.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v1 containers_v1.cpp
	$(CC) -Wall -Werror -o ./build/vectors vectors.cpp
	$(CC) -Wall -Werror -o ./build/containers_v2 containers_v2.cpp
	$(CC) -Wall -Werror -o ./build/containers_v3 containers_v3.cpp
	$(CC) -Wall -Werror -o ./build/containers_v4 containers_v4.cpp
	$(CC) -Wall -Werror -o ./build/containers_v5 containers_v5.cpp

containers1: containers_v1.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/containers_v1 containers_v1.cpp

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

vectors: vectors.cpp
	mkdir -p build
	$(CC) -Wall -Werror -o ./build/vectors vectors.cpp

clean:
	rm -f *.o
	rm -rf ./build