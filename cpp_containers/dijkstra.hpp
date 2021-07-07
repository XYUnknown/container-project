#include <iostream>
#include <vector>
#include <string>
#include <list>
 
#include <limits> // for numeric_limits

#include <queue>
#include <utility> // for pair
#include <fstream> // read graph data
#include <algorithm>
#include <iterator>
#include "containers.hpp"

typedef int vertex_t;
typedef double weight_t;
 
const weight_t max_weight = std::numeric_limits<double>::infinity();
 
struct neighbor {
    vertex_t target;
    weight_t weight;
    neighbor(vertex_t arg_target, weight_t arg_weight)
        : target(arg_target), weight(arg_weight) { }
};

typedef std::vector<std::vector<neighbor> > adjacency_list_t;
typedef std::pair<weight_t, vertex_t> weight_vertex_pair_t;

template<class C>
void DijkstraComputePaths(vertex_t source,
                          const adjacency_list_t &adjacency_list,
                          std::vector<weight_t> &min_distance,
                          std::vector<vertex_t> &previous)
{
    int n = adjacency_list.size();
    min_distance.clear();
    min_distance.resize(n, max_weight);
    min_distance[source] = 0;
    previous.clear();
    previous.resize(n, -1);
    C vertex_queue;
    
    // Approach One: the vertex queue is implemented using a std::priority_queue with underlying data structure being a vector
    if constexpr (std::is_same<C, std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>::value) {
        vertex_queue.push(std::make_pair(min_distance[source], source));
 
        while (!vertex_queue.empty()) 
        {
            weight_t dist = vertex_queue.top().first;
            vertex_t u = vertex_queue.top().second;
            vertex_queue.pop();
    
            // Because we leave old copies of the vertex in the priority queue
            // (with outdated higher distances), we need to ignore it when we come
            // across it again, by checking its distance against the minimum distance
            if (dist > min_distance[u])
                continue;
    
            // Visit each edge exiting u
            const std::vector<neighbor> &neighbors = adjacency_list[u];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++)
            {
                vertex_t v = neighbor_iter->target;
                weight_t weight = neighbor_iter->weight;
                weight_t distance_through_u = dist + weight;
                if (distance_through_u < min_distance[v]) {
                    min_distance[v] = distance_through_u;
                    previous[v] = u;
                    vertex_queue.push(std::make_pair(min_distance[v], v));
                }
            }
        }
    } else if constexpr (std::is_same<C, Container<weight_vertex_pair_t, TreeWrapper>>::value) {
        // Approach Two: the vertex queue is implemented using a balanced binary search tree without duplication
        vertex_queue.insert(std::make_pair(min_distance[source], source));
 
        while (!vertex_queue.empty()) 
        {
            weight_t dist = vertex_queue.begin()->first;
            vertex_t u = vertex_queue.begin()->second;
            vertex_queue.erase(vertex_queue.begin());

            if (dist > min_distance[u])
                continue;
    
            // Visit each edge exiting u
            const std::vector<neighbor> &neighbors = adjacency_list[u];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++)
            {
                vertex_t v = neighbor_iter->target;
                weight_t weight = neighbor_iter->weight;
                weight_t distance_through_u = dist + weight;
                if (distance_through_u < min_distance[v]) {
                    //vertex_queue.erase(std::make_pair(min_distance[v], v));
        
                    min_distance[v] = distance_through_u;
                    previous[v] = u;
                    vertex_queue.insert(std::make_pair(min_distance[v], v));
        
                }
            }
        }
    } else if constexpr (std::is_same<C, Container<weight_vertex_pair_t, std::vector>>::value) {
        // Approach Three: the vertex queue is implemented using an unsorted vector
        vertex_queue.push_back(std::make_pair(min_distance[source], source));
 
        while (!vertex_queue.empty()) 
        {
            auto min = std::min_element(vertex_queue.begin(), vertex_queue.end());
            weight_t dist = min->first;
            vertex_t u = min->second;
            vertex_queue.erase(min);

            // Because we leave old copies of the vertex in the priority queue
            // (with outdated higher distances), we need to ignore it when we come
            // across it again, by checking its distance against the minimum distance
            if (dist > min_distance[u])
                continue;
    
            // Visit each edge exiting u
            const std::vector<neighbor> &neighbors = adjacency_list[u];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++)
            {
                vertex_t v = neighbor_iter->target;
                weight_t weight = neighbor_iter->weight;
                weight_t distance_through_u = dist + weight;
                if (distance_through_u < min_distance[v]) {
                    min_distance[v] = distance_through_u;
                    previous[v] = u;
                    vertex_queue.push_back(std::make_pair(min_distance[v], v));
                }
            }
        }
    } else {
        // Approach Four: the vertex queue is implemented using a sorted vector (eager, descending)
        vertex_queue.insert(std::make_pair(min_distance[source], source));
 
        while (!vertex_queue.empty()) 
        {
            weight_t dist = vertex_queue.back().first;
            vertex_t u = vertex_queue.back().second;
            vertex_queue.pop_back();

            // Because we leave old copies of the vertex in the priority queue
            // (with outdated higher distances), we need to ignore it when we come
            // across it again, by checking its distance against the minimum distance
            if (dist > min_distance[u])
                continue;
    
            // Visit each edge exiting u
            const std::vector<neighbor> &neighbors = adjacency_list[u];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++)
            {
                vertex_t v = neighbor_iter->target;
                weight_t weight = neighbor_iter->weight;
                weight_t distance_through_u = dist + weight;
                if (distance_through_u < min_distance[v]) {
                    min_distance[v] = distance_through_u;
                    previous[v] = u;
                    vertex_queue.insert(std::make_pair(min_distance[v], v));
                }
            }
        }
    }

}

std::list<vertex_t> DijkstraGetShortestPathTo(
    vertex_t vertex, const std::vector<vertex_t> &previous)
{
    std::list<vertex_t> path;
    for ( ; vertex != -1; vertex = previous[vertex])
        path.push_front(vertex);
    return path;
}

adjacency_list_t constructGraph(std::string filename) {
    // Open the file:
    std::ifstream fin(filename);

    // Declare variables:
    int M, N, L;

    // Ignore headers and comments:
    while (fin.peek() == '%') fin.ignore(2048, '\n');

    // Read defining parameters:
    fin >> M >> N >> L;

    adjacency_list_t adjacency_list(M);

    // Read the data
    for (int l = 0; l < L; l++)
    {
        int m, n;
        double data;
        fin >> m >> n >> data;
        adjacency_list[m-1].push_back(neighbor(n-1, data)); // adjust index
    }

    fin.close();
    return adjacency_list;
}

std::pair<std::size_t, std::size_t> graphInfo(adjacency_list_t l) {
    auto numV = l.size();
    auto numE = 0;
    for (std::vector<neighbor> n : l) {
        numE += n.size();
    }
    return std::pair(numV, numE);
}

void printAdjList(adjacency_list_t l) {
    auto info = graphInfo(l);
    std::cout << "Number of vertices: " << info.first << " Number of edges: " << info.second << std::endl;
    for (auto i=0; i < l.size(); i++) {
        for (neighbor n : l[i]) {
            std::cout << "edge: {" << i << ", " << n.target << "} weight: " << n.weight << std::endl;
        }
    }
}
