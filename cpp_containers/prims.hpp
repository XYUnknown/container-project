#include <iostream>
#include <vector>
#include <string>
#include <list>
#include <queue>

template<class C>
void prims(std::vector<std::vector<std::pair<int, int>>> &adj_list, std::vector<bool> &visited, std::vector<int> &connection, std::vector<int> &value, /*std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>, std::greater<std::pair<int, int>>>*/ C &que) {
    que.push(std::make_pair(0, 0));  //push the weight required to insert the source node =0 and the node itself(i.e 1)
    value[0]=0;                 //minimum weight for source is 0  
    while (!que.empty()) {      
        int node = que.top().second;  //get the node
        visited[node] = true;         //as it is visited now change its value to true
        que.pop();                    
        for (auto neighbor : adj_list[node]) {   //we check for all its neighbors
            int weight = neighbor.second;        //get their weight
            int vertex = neighbor.first;         //get their index

            if (!visited[vertex] && value[vertex] > weight) {   //if the node is not visited and if its weight along this edge is less than the 
                value[vertex] = weight;                         //previous edge associated with it, then only we consider it
                connection[vertex] = node;
                que.push(std::make_pair(value[vertex], vertex));     //we update the values and then push it in the queue to examine its neighbors
            }
        }
    }
}

void print_graph(std::vector<int> &connection) {
    for (int i = 1; i < 6; ++i)
        printf("%d - %d\n", connection[i], i);  //print the connections
}

void makegraph(int m, int n, int wght, std::vector<std::vector<std::pair<int, int>>> &adj_list) {

    /* This function adds the edges and nodes to
      the graph in the form of an adjacency list */
    adj_list[m].push_back(std::make_pair(n, wght));     //make a pair of the node and its weight
    adj_list[n].push_back(std::make_pair(m, wght));     //we need to add it both ways i.e if a connects b, then b also connects a
}
