#include <iostream>
#include <vector>

class Unique {};
class Sorted {};

template<class T, class P>
struct Vec;

template <class T>
struct Vec<T, void> : public std::vector<T> {
    bool contains(T t) {
        return find(std::vector<T>::begin(), std::vector<T>::end(), t) != std::vector<T>::end();
    }
};

template <class T>
struct Vec<T, Unique> : Vec<T, void> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Vec<T, void>::push_back(t);
        }
    }
};

template <class T>
struct Vec<T, Sorted> : Vec<T, void> {
    void push_back(T t) {
        // find the insertion point
        auto pos = std::lower_bound(std::vector<T>::begin(), std::vector<T>::end(), t);
        std::vector<T>::insert(pos, t);
    }
};

int main() {
    Vec<int, void> v1;
    v1.push_back(1);
    v1.push_back(1);

    Vec<int, Unique> v2;
    v2.push_back(1);
    v2.push_back(1);

    Vec<int, Sorted> v3;
    v3.push_back(3);
    v3.push_back(1);
    v3.push_back(3);

    std::cout << "Default Vec" << std::endl;
    std::cout << "Size: " << v1.size() << std::endl;
    for (auto it=v1.begin(); it<v1.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    std::cout << "Unique Vec" << std::endl;
    std::cout << "Size: " << v2.size() << std::endl;
    for (auto it=v2.begin(); it<v2.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    std::cout << "Sorted Vec" << std::endl;
    std::cout << "Size: " << v3.size() << std::endl;
    for (auto it=v3.begin(); it<v3.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';

    return 0;
}