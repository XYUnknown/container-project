#include <iostream>
#include <vector>
#include <list>

class Unique {};
class Sorted {};

template <class P1, class P2>
struct And;

// We can try making a container where we can sepecify what concrete container type and 
// semantic properties are
// C - concrete container type; P - semantic property(-ies)
template<class T, class C, class P>
struct Container;

// Vectors
template<class T>
struct Container<T, std::vector<T>, void> : std::vector<T> {
    bool contains(T t) {
        return std::find(std::vector<T>::begin(), std::vector<T>::end(), t) != std::vector<T>::end();
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

// Lists
template <class T>
struct Container<T, std::list<T>, void> : public std::list<T> {
    bool contains(T t) {
        return std::find(std::list<T>::begin(), std::list<T>::end(), t) != std::list<T>::end();
    }
};

template <class T>
struct Container<T, std::list<T>, Unique> : public virtual Container<T, std::list<T>, void> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, void>::push_back(t);
        }
    }

    void push_front(T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, void>::push_front(t);
        }
    }

    auto insert(typename std::list<T>::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, void>::insert(pos, t);
        }
    }
};

template <class T>
struct Container<T, std::list<T>, Sorted> : public virtual Container<T, std::list<T>, void> {
    void push_back(T t) {
        this->insert(this->end(), t);
    }

    void push_front(T t) {
        this->insert(this->begin(), t);
    }

    auto insert(typename std::list<T>::iterator pos, T t) {
        // we cannot access element by position in a list
        // therefore we need to search the whole list
        auto pos_i = std::lower_bound(this->begin(), pos, t);
        if (pos_i == pos) {
            pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, std::list<T>, void>::insert(pos_i, t);
        } else {
            Container<T, std::list<T>, void>::insert(pos_i, t);
        }
    }
};

template <class T>
struct Container<T, std::list<T>, And<Sorted, Unique>> : Container<T, std::list<T>, Sorted>, Container<T, std::list<T>, Unique> {
    void push_back(T t) {
        this->insert(this->end(), t);
    }

    void push_front(T t) {
        this->insert(this->begin(), t);
    }

    auto insert(typename std::list<T>::iterator pos, T t) {
        auto pos_i = std::lower_bound(this->begin(), pos, t);
        if (pos_i == pos) {
            pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, std::list<T>, Unique>::insert(pos_i, t);
        } else {
            Container<T, std::list<T>, Unique>::insert(pos_i, t);
        }
    }
};

template <class T>
struct Container<T, std::list<T>, And<Unique, Sorted>> : Container<T, std::list<T>, Unique>, Container<T, std::list<T>, Sorted> {
    void push_back(T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, Sorted>::push_back(t);
        }
    }

    void push_front(T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, Sorted>::push_front(t);
        }
    }

    auto insert(typename std::list<T>::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, std::list<T>, Sorted>::insert(pos, t);
        }
    }
};

// Helpers
template<class C>
void print_container(C c) {
    std::cout << "Size: " << c.size() << std::endl;
    for (auto it=c.begin(); it!=c.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';
}

int main() {
    // Vectors
    Container<int, std::vector<int>, void> c1;
    c1.push_back(1);
    c1.push_back(1);
    std::cout << "Container for default vector" << std::endl;
    print_container(c1);

    Container<int, std::vector<int>, Unique> c2;
    c2.push_back(1);
    c2.push_back(1);
    c2.insert(c2.begin(), 1);
    std::cout << "Container for unique vector" << std::endl;
    print_container(c2);

    Container<int, std::vector<int>, Sorted> c3;
    c3.push_back(3);
    c3.push_back(1);
    c3.push_back(3);
    c3.insert(c3.begin(), 4);
    std::cout << "Container for sorted vector" << std::endl;
    print_container(c3);

    Container<int, std::vector<int>, And<Sorted, Unique>> c4;
    c4.push_back(3);
    c4.push_back(1);
    c4.push_back(3);
    c4.insert(c4.begin(), 4);
    c4.insert(c4.begin(), 4);
    std::cout << "Container for sorted unique vector" << std::endl;
    print_container(c4);

    Container<int, std::vector<int>, And<Unique, Sorted>> c5;
    c5.push_back(3);
    c5.push_back(1);
    c5.push_back(3);
    c5.insert(c5.begin(), 4);
    c5.insert(c5.begin(), 4);
    std::cout << "Container for unique sorted vector" << std::endl;
    print_container(c5);

    // Lists
    Container<int, std::list<int>, void> c6;
    c6.push_back(1);
    c6.push_back(1);
    std::cout << "Container for default list" << std::endl;
    print_container(c6);

    Container<int, std::list<int>, Unique> c7;
    c7.push_back(3);
    c7.push_back(1);
    c7.push_front(1);
    c7.insert(c7.begin(), 1);
    c7.push_back(1);
    std::cout << "Container for unique list" << std::endl;
    print_container(c7);

    Container<int, std::list<int>, Sorted> c8;
    c8.push_back(3);
    c8.push_back(1);
    c8.push_front(5);
    c8.insert(c8.begin(), 6);
    c8.push_back(1);
    std::cout << "Container for sorted list" << std::endl;
    print_container(c8);

    Container<int, std::list<int>, And<Sorted, Unique>> c9;
    c9.push_back(3);
    c9.push_back(1);
    c9.push_front(5);
    c9.insert(c9.begin(), 6);
    c9.push_back(1);
    std::cout << "Container for sorted unique list" << std::endl;
    print_container(c9);

    Container<int, std::list<int>, And<Unique, Sorted>> c10;
    c10.push_back(3);
    c10.push_back(1);
    c10.push_front(5);
    c10.insert(c10.begin(), 6);
    c10.push_back(1);
    std::cout << "Container for unique sorted list" << std::endl;
    print_container(c10);

    return 0;
}