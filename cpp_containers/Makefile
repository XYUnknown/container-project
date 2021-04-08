all: containers.cpp vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/containers containers.cpp
	clang++ -Wall -Werror -std=c++17 -o ./build/vectors vectors.cpp

containers: containers.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/containers containers.cpp

vectors: vectors.cpp
	mkdir -p build
	clang++ -Wall -Werror -std=c++17 -o ./build/vectors vectors.cpp

clean:
	rm -f *.o ./build/containers *~