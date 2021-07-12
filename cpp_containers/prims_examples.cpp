#include "prims.hpp"

int main() {
    int V = 6;
    adjacency_list_t adj_list(V); //to make the graph
    std::vector<bool> visited(V,false);  // to keep a track if the node has been already visited or not. Initially all are false as no node is visited
    std::vector<vertex_t> connection(V, -1);  // to track the final connections that the MST has
    std::vector<weight_t> value (V, max_weight); // to store the minimum weight value possible for a node

    makegraph(4, 3, 5, adj_list);   //insert the node and edge
    makegraph(4, 0, 3, adj_list);   //insert the node and edge
    makegraph(0, 1, 3, adj_list);   //insert the node and edge
    makegraph(0, 3, 7, adj_list);   //insert the node and edge
    makegraph(1, 4, 11, adj_list);  //insert the node and edge
    makegraph(5, 3, 1, adj_list);   //insert the node and edge
    makegraph(4, 5, 7, adj_list);   //insert the node and edge
    makegraph(2, 0, 3, adj_list);   //insert the node and edge
    makegraph(2, 1, 7, adj_list);   //insert the node and edge

    prims<std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>(adj_list, visited, connection, value);  //call the function the perform the minimum spanning tree algorithm
    print_graph(connection);        //print the final minimum spanning tree

    return 0;
}
