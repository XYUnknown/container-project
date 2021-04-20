all: containers.cpp containers_v2.cpp vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/containers containers.cpp
	clang++ -Wall -Werror -std=c++17 -o ./build/vectors vectors.cpp
	clang++ -Wall -Werror -std=c++17 -o ./build/containers_v2 containers_v2.cpp

containers: containers.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/containers containers.cpp

containers2: containers_v2.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/containers_v2 containers_v2.cpp

vectors: vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/vectors vectors.cpp

clean:
	rm -f *.o ./build/containers *~