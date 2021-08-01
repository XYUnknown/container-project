#include "powerset.hpp"

int main()
{
    Container<std::size_t, HashSetWrapper, Iterable> set;
    std::size_t size = 20;
    for (std::size_t i=0; i < size; i++) {
        set.insert((std::size_t) 2 * i);
    }
    
    auto pset = powerset<Container<std::size_t, HashSetWrapper, Iterable>, std::vector, Iterable>(set);
 
    std::cout << "Number of powersets: " << pset.size() << std::endl;
    return 0;
}