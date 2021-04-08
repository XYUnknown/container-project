#include <iostream>
#include <vector>

class Unique {};
class Sorted {};

// We can try making a container where we can sepecify what concrete container type and 
// semantic properties 
template<class T, class C, class P>
struct Container;

template<class T>
struct Container<T, std::vector<T>, void> : public std::vector<T> {
    bool contains(T t) {
        return find(std::vector<T>::begin(), std::vector<T>::end(), t) != std::vector<T>::end();
    }
};

template <class T>
struct Container<T, std::vector<T>, Unique> : Container<T, std::vector<T>, void> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Container<T, std::vector<T>, void>::push_back(t);
        }
    }
};

template <class T>
struct Container<T, std::vector<T>, Sorted> : Container<T, std::vector<T>, void> {
    void push_back(T t) {
        auto pos = std::lower_bound(std::vector<T>::begin(), std::vector<T>::end(), t);
        Container<T, std::vector<T>, void>::insert(pos, t);
    }
};

int main() {
    Container<int, std::vector<int>, void> c1;
    c1.push_back(1);
    c1.push_back(1);
    std::cout << "Container for default vector" << std::endl;
    std::cout << "Size: " << c1.size() << std::endl;
    for (auto it=c1.begin(); it<c1.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    Container<int, std::vector<int>, Unique> c2;
    c2.push_back(1);
    c2.push_back(1);
    std::cout << "Container for unique vector" << std::endl;
    std::cout << "Size: " << c2.size() << std::endl;
    for (auto it=c2.begin(); it<c2.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    Container<int, std::vector<int>, Sorted> c3;
    c3.push_back(3);
    c3.push_back(1);
    c3.push_back(3);
    std::cout << "Container for unique vector" << std::endl;
    std::cout << "Size: " << c3.size() << std::endl;
    for (auto it=c3.begin(); it<c3.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    return 0;
}