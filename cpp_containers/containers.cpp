#include <iostream>
#include <vector>

class Unique {};
class Sorted {};

template <class P1, class P2>
struct And;

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
struct Container<T, std::vector<T>, Unique> : public virtual Container<T, std::vector<T>, void> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Container<T, std::vector<T>, void>::push_back(t);
        }
    }

    auto insert(typename std::vector<T>::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, std::vector<T>, void>::insert(pos, t);
        }
    }
};

template <class T>
struct Container<T, std::vector<T>, Sorted> : public virtual Container<T, std::vector<T>, void> {
    void push_back(T t) {
        auto pos = std::lower_bound(this->begin(), this->end(), t);
        Container<T, std::vector<T>, void>::insert(pos, t);
    }

    auto insert(typename std::vector<T>::iterator pos, T t) {
        if (this->at(pos-this->begin()) >= t) {
            auto pos_i = std::lower_bound(this->begin(), pos, t);
            Container<T, std::vector<T>, void>::insert(pos_i, t);
        } else {
            auto pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, std::vector<T>, void>::insert(pos_i, t);
        }
    }
};

template <class T>
struct Container<T, std::vector<T>, And<Sorted, Unique>> : Container<T, std::vector<T>, Sorted>, Container<T, std::vector<T>, Unique> {
    void push_back(T t) {
        auto pos = std::lower_bound(this->begin(), this->end(), t);
        Container<T, std::vector<T>, Unique>::insert(pos, t);
    }

    auto insert(typename std::vector<T>::iterator pos, T t) {
        if (this->at(pos-this->begin()) >= t) {
            auto pos_i = std::lower_bound(this->begin(), pos, t);
            Container<T, std::vector<T>, Unique>::insert(pos_i, t);
        } else {
            auto pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, std::vector<T>, Unique>::insert(pos_i, t);
        }
    }
};

template <class T>
struct Container<T, std::vector<T>, And<Unique, Sorted>> : Container<T, std::vector<T>, Unique>, Container<T, std::vector<T>, Sorted> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Container<T, std::vector<T>, Sorted>::push_back(t);
        } 
    }

    auto insert(typename std::vector<T>::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, std::vector<T>, Sorted>::insert(pos, t);
        } 
    }
};

void print_vector(std::vector<int> v) {
    std::cout << "Size: " << v.size() << std::endl;
    for (auto it=v.begin(); it<v.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';
}

int main() {
    Container<int, std::vector<int>, void> c1;
    c1.push_back(1);
    c1.push_back(1);
    std::cout << "Container for default vector" << std::endl;
    print_vector(c1);

    Container<int, std::vector<int>, Unique> c2;
    c2.push_back(1);
    c2.push_back(1);
    c2.insert(c2.begin(), 1);
    std::cout << "Container for unique vector" << std::endl;
    print_vector(c2);

    Container<int, std::vector<int>, Sorted> c3;
    c3.push_back(3);
    c3.push_back(1);
    c3.push_back(3);
    c3.insert(c3.begin(), 4);
    std::cout << "Container for sorted vector" << std::endl;
    print_vector(c3);

    Container<int, std::vector<int>, And<Sorted, Unique>> c4;
    c4.push_back(3);
    c4.push_back(1);
    c4.push_back(3);
    c4.insert(c4.begin(), 4);
    c4.insert(c4.begin(), 4);
    std::cout << "Container for sorted unique vector" << std::endl;
    print_vector(c4);

    Container<int, std::vector<int>, And<Unique, Sorted>> c5;
    c5.push_back(3);
    c5.push_back(1);
    c5.push_back(3);
    c5.insert(c5.begin(), 4);
    c5.insert(c5.begin(), 4);
    std::cout << "Container for unique sorted vector" << std::endl;
    print_vector(c5);

    return 0;
}