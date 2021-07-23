#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>
#include <list>
 
#include <limits> // for numeric_limits

#include <queue>
#include <utility> // for pair
#include <fstream> // read graph data
#include <algorithm>
#include <iterator>
#include <random>

typedef int vertex_t;
typedef double weight_t;

struct neighbor {
    vertex_t target;
    weight_t weight;
    neighbor(vertex_t arg_target, weight_t arg_weight)
        : target(arg_target), weight(arg_weight) { }
};

typedef std::vector<std::vector<neighbor>> adjacency_list_t;
typedef std::pair<weight_t, vertex_t> weight_vertex_pair_t;

adjacency_list_t generateUndirectedWeightGraph(int numV, int numE, int maxWeight) {
    auto hash = [](const std::pair<int, int>& p){ return p.first * 31 + p.second; };
    std::unordered_set<std::pair<int, int>, decltype(hash)> edges; // for duplication checking
  
    std::minstd_rand generator;
    generator.seed(numV);
    
    adjacency_list_t adjacency_list(numV);
    // to ensure the graph is connected
    for (int v=0; v<numV-1; v++) {
        std::pair<int, int> e = std::make_pair(v, v+1);
        edges.insert(e);
        weight_t w = (double) (generator() % maxWeight);
        adjacency_list[v].push_back(neighbor(v+1, w));
        adjacency_list[v+1].push_back(neighbor(v, w));
    }
  
    // generate the rest of edges
    for (int j=0; j<numE-(numV-1); j++) {
        vertex_t m = generator() % numV;
        vertex_t n = generator() % numV;
        while (n == m) {
            n = generator() % numV;
        }
        std::pair<int, int> e = std::make_pair(m, n);
        std::pair<int, int> er = std::make_pair(n, m);
  
        // Search for a random "new" edge every time
        while (edges.contains(e) || edges.contains(er)) {
            m = generator() % numV;
            n = generator() % numV;
            while (n == m) {
                n = generator() % numV;
            }
            e = std::make_pair(m, n);
            er = std::make_pair(n, m);
        }
        edges.insert(e);
        weight_t w = (double) (generator() % maxWeight);
        adjacency_list[m].push_back(neighbor(n, w));
        adjacency_list[n].push_back(neighbor(m, w));
    }
    return adjacency_list;
}