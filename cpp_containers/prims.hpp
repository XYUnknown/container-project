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
#include "graph.hpp"
 
const weight_t max_weight = std::numeric_limits<double>::infinity();

void makegraph(int m, int n, weight_t wght, adjacency_list_t &adj_list) {

    /* This function adds the edges and nodes to
      the graph in the form of an adjacency list */
    adj_list[m].push_back(neighbor(n, wght));     //make a pair of the node and its weight
    adj_list[n].push_back(neighbor(m, wght));     //we need to add it both ways i.e if a connects b, then b also connects a
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
        makegraph(m-1, n-1, data, adjacency_list);
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
    return std::pair(numV, numE / 2);
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

template<class C>
void prims(const adjacency_list_t &adj_list, 
           std::vector<bool> &visited, 
           std::vector<vertex_t> &connection, 
           std::vector<weight_t> &value) {
    int n = adj_list.size();
    visited.clear();
    visited.resize(n, false);
    connection.clear();
    connection.resize(n, -1);
    value.clear();
    value.resize(n, max_weight);
    C que;
    if constexpr (std::is_same<C, std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>::value) {
        que.push(std::make_pair(0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
        value[0] = 0;                 //minimum weight for source is 0  
        while (!que.empty()) {      
            vertex_t node = que.top().second;  //get the node
            que.pop();
            if(visited[node] == true){
                continue;
            }
            visited[node] = true;
            const std::vector<neighbor> &neighbors = adj_list[node];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++) {   //we check for all its neighbors
                weight_t weight = neighbor_iter->weight;        //get their weight
                vertex_t vertex = neighbor_iter->target;         //get their index

                if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                    value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                    connection[vertex] = node;
                    que.push(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
                }
            }
        }
    } else if constexpr (std::is_same<C, Container<weight_vertex_pair_t, TreeWrapper>>::value) { // balanced binary search tree
        que.insert(std::make_pair(0.0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
        value[0] = 0.0;                 //minimum weight for source is 0  
        while (!que.empty()) {      
            vertex_t node = que.begin()->second;  //get the node
            que.erase(que.begin());
            if(visited[node] == true){
                continue;
            }
            visited[node] = true;           
            const std::vector<neighbor> &neighbors = adj_list[node];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++) {   //we check for all its neighbors
                weight_t weight = neighbor_iter->weight;        //get their weight
                vertex_t vertex = neighbor_iter->target;        //get their index

                if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                    value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                    connection[vertex] = node;
                    que.insert(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
                }
            }
        }
    } else if constexpr (std::is_same<C, Container<weight_vertex_pair_t, std::vector>>::value) {
        que.push_back(std::make_pair(0.0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
        value[0] = 0.0;                 //minimum weight for source is 0  
        while (!que.empty()) {   
            auto min = std::min_element(que.begin(), que.end());   
            vertex_t node = min->second;  //get the node
            que.erase(min);
            if(visited[node] == true){
                continue;
            }
            visited[node] = true;           
            const std::vector<neighbor> &neighbors = adj_list[node];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++) {   //we check for all its neighbors
                weight_t weight = neighbor_iter->weight;        //get their weight
                vertex_t vertex = neighbor_iter->target;        //get their index

                if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                    value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                    connection[vertex] = node;
                    que.push_back(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
                }
            }
        }
    } else {
        que.insert(std::make_pair(0.0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
        value[0] = 0.0;                 //minimum weight for source is 0  
        while (!que.empty()) {      
            vertex_t node = que.back().second;  //get the node
            que.pop_back();
            if(visited[node] == true){
                continue;
            }
            visited[node] = true;                    
            const std::vector<neighbor> &neighbors = adj_list[node];
            for (std::vector<neighbor>::const_iterator neighbor_iter = neighbors.begin();
                neighbor_iter != neighbors.end();
                neighbor_iter++) {   //we check for all its neighbors
                weight_t weight = neighbor_iter->weight;        //get their weight
                vertex_t vertex = neighbor_iter->target;        //get their index

                if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                    value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                    connection[vertex] = node;
                    que.insert(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
                }
            }
        }
    }
}

void print_graph(std::vector<vertex_t> &connection) {
    for (int i = 1; i < connection.size(); ++i)
        printf("%d - %d\n", connection[i], i);  //print the connections
}
