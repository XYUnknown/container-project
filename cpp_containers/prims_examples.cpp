#include "prims.hpp"

int main() {
    int V = 6;
    adjacency_list_t adj_list(V); //to make the graph
    std::vector<bool> visited; // to keep a track if the node has been already visited or not. Initially all are false as no node is visited
    std::vector<vertex_t> connection; // to track the final connections that the MST has
    std::vector<weight_t> value; // to store the minimum weight value possible for a node

    //insert the node and edge
    makegraph(4, 3, 5, adj_list);   
    makegraph(4, 0, 3, adj_list); 
    makegraph(0, 1, 3, adj_list);
    makegraph(0, 3, 7, adj_list);
    makegraph(1, 4, 11, adj_list);
    makegraph(5, 3, 1, adj_list);
    makegraph(4, 5, 7, adj_list);
    makegraph(2, 0, 3, adj_list);
    makegraph(2, 1, 7, adj_list);

    adjacency_list_t adjList = constructGraph("./matrix/uw800s.mtx");
    adjacency_list_t randomG = generateUndirectedWeightGraph(10, 20, 30);
    printAdjList(randomG);
    //printAdjList(adjList);
    //prims<std::priority_queue<weight_vertex_pair_t, std::vector<weight_vertex_pair_t>, std::greater<weight_vertex_pair_t>>>(adjList, visited, connection, value);  //call the function the perform the minimum spanning tree algorithm
    //prims<Container<weight_vertex_pair_t, TreeWrapper>>(adj_list, visited, connection, value); 
    //prims<Container<weight_vertex_pair_t, std::vector, SortedOnAccess<weight_vertex_pair_t, std::greater<weight_vertex_pair_t>>>>(adjList, visited, connection, value); 
    prims<Container<weight_vertex_pair_t, std::vector>>(randomG, visited, connection, value); 
    print_graph(connection);        //print the final minimum spanning tree


    return 0;
}
