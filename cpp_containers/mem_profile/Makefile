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

all: powerset_set.cpp powerset_unordered_set.cpp powerset_vector.cpp powerset_uvector_eager.cpp powerset_uvector_lazy.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_14 powerset_set_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_16 powerset_set_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_18 powerset_set_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set powerset_set.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_14 powerset_unordered_set_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_16 powerset_unordered_set_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_18 powerset_unordered_set_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set powerset_unordered_set.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_14 powerset_vector_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_16 powerset_vector_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_18 powerset_vector_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector powerset_vector.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_14 powerset_uvector_eager_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_16 powerset_uvector_eager_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_18 powerset_uvector_eager_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager powerset_uvector_eager.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_14 powerset_uvector_lazy_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_16 powerset_uvector_lazy_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_18 powerset_uvector_lazy_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy powerset_uvector_lazy.cpp
	
set: powerset_set.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_14 powerset_set_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_16 powerset_set_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set_18 powerset_set_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_set powerset_set.cpp

unorderedset: powerset_unordered_set.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_14 powerset_unordered_set_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_16 powerset_unordered_set_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set_18 powerset_unordered_set_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_unordered_set powerset_unordered_set.cpp

vector: powerset_vector.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_14 powerset_vector_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_16 powerset_vector_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector_18 powerset_vector_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_vector powerset_vector.cpp

uvectore: powerset_uvector_eager.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_14 powerset_uvector_eager_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_16 powerset_uvector_eager_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager_18 powerset_uvector_eager_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_eager powerset_uvector_eager.cpp

uvectorl: powerset_uvector_lazy.cpp
	mkdir -p build
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_14 powerset_uvector_lazy_14.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_16 powerset_uvector_lazy_16.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy_18 powerset_uvector_lazy_18.cpp
	$(CC) -Wall -Werror -O2 -o ./build/powerset_uvector_lazy powerset_uvector_lazy.cpp

clean:
	rm -rf ./build
