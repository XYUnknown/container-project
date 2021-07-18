CC = clang++ -std=c++20
CMP = /afs/inf.ed.ac.uk/user/s16/s1673055/tools/clang+llvm-12.0.0-x86_64-linux-sles12.4
INCLUDE = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/include/benchmark
LIB = /afs/inf.ed.ac.uk/user/s16/s1673055/usr/lib
CON = /Users/yukiko/Documents/container-project/cpp_containers/includes

EXT =

OS := $(shell uname)
ifeq ($(OS), Darwin)
	CC = clang++ -std=c++20
else
	CC = $(CMP)/bin/clang++ -std=c++20 -stdlib=libc++ -nostdinc++ -I$(CMP)/include/c++/v1 -L$(CMP)/lib -Wl,-rpath,$(CMP)/lib
	EXT = -I$(INCLUDE) -L$(LIB)
endif

all: benchmark_lookup.cc benchmark_insertion.cc benchmark_insertion_lookup.cc benchmark_unique.cc benchmark_pq_single.cc benchmark_pq_push.cc benchmark_pq_pop.cc benchmark_pq_cons.cc benchmark_dijkstra.cc benchmark_prims.cc benchmark_prims_s.cc benchmark_prims_d.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_lookup
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_lookup_o2
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_lookup_o3

	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_insertion
	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_insertion_o2
	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_insertion_o3

	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_insertion_lookup
	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_insertion_lookup_o2
	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_insertion_lookup_o3

	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_unique
	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_unique_o2
	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_unique_o3

	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_single
	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_single_o2
	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_single_o3

	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_push
	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_push_o2
	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_push_o3

	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_pop
	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_pop_o2
	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_pop_o3

	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_push_pop
	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_push_pop_o2
	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_push_pop_o3

	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_cons
	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_cons_o2
	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_cons_o3

	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_dijkstra
	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_dijkstra_o2
	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_dijkstra_o3

	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims
	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_o2
	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_o3

	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims_s
	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_s_o2
	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_s_o3

	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims_d
	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_d_o2
	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_d_o3

benchlookup: benchmark_lookup.cc 
	mkdir -p build
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_lookup
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_lookup_o2
	$(CC) $(EXT) benchmark_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_lookup_o3

benchinsertion: benchmark_insertion.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_insertion
	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_insertion_o2
	$(CC) $(EXT) benchmark_insertion.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_insertion_o3

benchinslook: benchmark_insertion_lookup.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_insertion_lookup
	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_insertion_lookup_o2
	$(CC) $(EXT) benchmark_insertion_lookup.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_insertion_lookup_o3

benchunique: benchmark_unique.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_unique
	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_unique_o2
	$(CC) $(EXT) benchmark_unique.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_unique_o3

benchpq: benchmark_pq_single.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_single
	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_single_o2
	$(CC) $(EXT) benchmark_pq_single.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_single_o3

benchpqpush: benchmark_pq_push.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_push
	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_push_o2
	$(CC) $(EXT) benchmark_pq_push.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_push_o3

benchpqpop: benchmark_pq_pop.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_pop
	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_pop_o2
	$(CC) $(EXT) benchmark_pq_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_pop_o3

benchpqpushpop: benchmark_pq_push_pop.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_push_pop
	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_push_pop_o2
	$(CC) $(EXT) benchmark_pq_push_pop.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_push_pop_o3

benchpqcons: benchmark_pq_cons.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_pq_cons
	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_pq_cons_o2
	$(CC) $(EXT) benchmark_pq_cons.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_pq_cons_o3

benchdijkstra: benchmark_dijkstra.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_dijkstra
	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_dijkstra_o2
	$(CC) $(EXT) benchmark_dijkstra.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_dijkstra_o3

benchprims: benchmark_prims.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims
	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_o2
	$(CC) $(EXT) benchmark_prims.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_o3

benchprimss: benchmark_prims_s.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims_s
	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_s_o2
	$(CC) $(EXT) benchmark_prims_s.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_s_o3

benchprimsd: benchmark_prims_d.cc
	mkdir -p build
	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -o ./build/benchmark_prims_d
	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O2 -o ./build/benchmark_prims_d_o2
	$(CC) $(EXT) benchmark_prims_d.cc -I$(CON) -isystem benchmark/include -lbenchmark -lpthread -O3 -o ./build/benchmark_prims_d_o3

clean:
	rm -f *.o
	rm -rf ./build