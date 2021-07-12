#include <iostream>
#include <vector>
#include <string>
#include <list>
#include <queue>

typedef int vertex_t;
typedef double weight_t;
 
const weight_t max_weight = std::numeric_limits<double>::infinity();
 
struct neighbor {
    vertex_t target;
    weight_t weight;
    neighbor(vertex_t arg_target, weight_t arg_weight)
        : target(arg_target), weight(arg_weight) { }
};

typedef std::vector<std::vector<neighbor>> adjacency_list_t;
typedef std::pair<weight_t, vertex_t> weight_vertex_pair_t;

template<class C>
void prims(const adjacency_list_t &adj_list, 
           std::vector<bool> &visited, 
           std::vector<vertex_t> &connection, 
           std::vector<weight_t> &value) {
    C que;
    que.push(std::make_pair(0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
    value[0]=0;                 //minimum weight for source is 0  
    while (!que.empty()) {      
        vertex_t node = que.top().second;  //get the node
        visited[node] = true;         //as it is visited now change its value to true
        que.pop();                    
        for (auto neighbor : adj_list[node]) {   //we check for all its neighbors
            weight_t weight = neighbor.weight;        //get their weight
            vertex_t vertex = neighbor.target;         //get their index

            if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                connection[vertex] = node;
                que.push(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
            }
        }
    }
}

void print_graph(std::vector<vertex_t> &connection) {
    for (int i = 1; i < 6; ++i)
        printf("%d - %d\n", connection[i], i);  //print the connections
}

void makegraph(int m, int n, weight_t wght, adjacency_list_t &adj_list) {

    /* This function adds the edges and nodes to
      the graph in the form of an adjacency list */
    adj_list[m].push_back(neighbor(n, wght));     //make a pair of the node and its weight
    adj_list[n].push_back(neighbor(m, wght));     //we need to add it both ways i.e if a connects b, then b also connects a
}
