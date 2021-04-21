#include <iostream>
#include <vector>
#include <list>
#include <typeinfo>

class Unique {};
class Sorted {};

template <class P1, class P2>
struct And;

template<class T, class C, class P>
struct Container;

template<class T> struct dependent_false : std::false_type {};

template<class T, class C>
struct Container<T, C, void> : C {
    auto at(size_t pos) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::at(pos);
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            static_assert(dependent_false<T>::value, "at is only defined for vectors");
        } else {
            // at compilation level
            // ref : https://en.cppreference.com/w/cpp/language/if
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto size() {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::size();
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            return std::list<T>::size();
        } else {
            // at compilation level
            // ref : https://en.cppreference.com/w/cpp/language/if
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto empty() {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::empty();
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            return std::list<T>::empty();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto push_back(T t) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            std::vector<T>::push_back(t);
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            std::list<T>::push_back(t);
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto push_front(T t) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            static_assert(dependent_false<T>::value, "push_front is only defined for list");
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            std::list<T>::push_front(t);
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto insert(typename C::iterator pos, T t) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            std::vector<T>::insert(pos, t);
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            std::list<T>::insert(pos, t);
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto begin() {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::begin();
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            return std::list<T>::begin();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto end() {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::end();
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            return std::list<T>::end();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    bool contains(T t) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::find(std::vector<T>::begin(), std::vector<T>::end(), t) != std::vector<T>::end();
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            return std::find(std::list<T>::begin(), std::list<T>::end(), t) != std::list<T>::end();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    void print() {
        if constexpr (std::is_same<C, std::vector<T>>::value || std::is_same<C, std::list<T>>::value)  {
            std::cout << "Size: " << this->size() << std::endl;
            for (auto it=this->begin(); it!=this->end(); it++)
                std::cout << ' ' << *it;
            std::cout << '\n';
            std::cout << '\n';
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }
};

template<class T, class C>
struct Container<T, C, Unique> : public virtual Container<T, C, void> {
    auto push_back(T t) {
        if (!this->contains(t)) {
            Container<T, C, void>::push_back(t);
        }
    }

    auto push_front(T t) {
        if (!this->contains(t)) {
            Container<T, C, void>::push_front(t);
        }
    }

    auto insert(typename C::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, C, void>::insert(pos, t);
        }
    }
};

template<class T, class C>
struct Container<T, C, Sorted> : public virtual Container<T, C, void> {
    auto push_back(T t) {
        this->insert(this->end(), t);
    }

    auto push_front(T t) {
        if constexpr (std::is_same<C, std::list<T>>::value) {
            this->insert(this->begin(), t);
        } else {
            static_assert(dependent_false<T>::value, "push_front is only defined for list");
        }
    }

    auto insert(typename C::iterator pos, T t) {
        auto pos_i = std::lower_bound(this->begin(), pos, t);
        if (pos_i == pos) {
            pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, C, void>::insert(pos_i, t);
        } else {
            Container<T, C, void>::insert(pos_i, t);
        }
    }
};


template<class T, class C>
struct Container<T, C, And<Sorted, Unique>> : Container<T, C, Sorted>, Container<T, C, Unique> {
    auto push_back(T t) {
        if (!this->contains(t)) {
            Container<T, C, Sorted>::push_back(t);
        }
    }

    auto push_front(T t) {
        if (!this->contains(t)) {
            Container<T, C, Sorted>::push_front(t);
        }
    }

    auto insert(typename C::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, C, Sorted>::insert(pos, t);
        }
    }
};

template<class T, class C>
struct Container<T, C, And<Unique, Sorted>> : Container<T, C, Unique>, Container<T, C, Sorted> {
    auto push_back(T t) {
        this->insert(this->end(), t);
    }

    auto push_front(T t) {
        if constexpr (std::is_same<C, std::list<T>>::value) {
            this->insert(this->begin(), t);
        } else {
            static_assert(dependent_false<T>::value, "push_front is only defined for list");
        }
    }

    auto insert(typename C::iterator pos, T t) {
        auto pos_i = std::lower_bound(this->begin(), pos, t);
        if (pos_i == pos) {
            pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, C, Unique>::insert(pos_i, t);
        } else {
            Container<T, C, Unique>::insert(pos_i, t);
        }
    }
};

int main() {
    // std::cout << std::is_same<std::vector<int>, std::vector<int>>::value << ' ';
    Container<int, std::vector<int>, void> v1;
    v1.push_back(3);
    v1.insert(v1.begin(), 5);
    std::cout << typeid(v1).name() << std::endl;
    std::cout << "Container for default vector" << std::endl;
    v1.print();

    Container<int, std::list<int>, void> l1;
    l1.push_back(3);
    l1.push_front(6);
    l1.insert(l1.begin(), 5);
    std::cout << typeid(l1).name() << std::endl;
    std::cout << "Container for default list" << std::endl;
    l1.print();

    Container<int, std::vector<int>, Unique> v2;
    v2.push_back(1);
    v2.push_back(1);
    // v2.push_front(1); // invalid
    v2.insert(v2.begin(), 1);
    std::cout << "Container for unique vector" << std::endl;
    v2.print();

    Container<int, std::list<int>, Unique> l2;
    l2.push_back(1);
    l2.push_back(1);
    l2.push_front(1);
    l2.insert(l2.begin(), 1);
    std::cout << "Container for unique list" << std::endl;
    l2.print();

    Container<int, std::vector<int>, Sorted> v3;
    v3.push_back(6);
    v3.push_back(1);
    //v3.push_front(4);
    v3.insert(v3.begin(), 1);
    std::cout << "Container for sorted vector" << std::endl;
    v3.print();

    Container<int, std::list<int>, Sorted> l3;
    l3.push_back(6);
    l3.push_back(1);
    l3.push_front(4);
    l3.insert(l3.begin(), 1);
    std::cout << "Container for sorted list" << std::endl;
    l3.print();

    Container<int, std::vector<int>, And<Sorted, Unique>> v4;
    v4.push_back(6);
    v4.push_back(1);
    v4.insert(v4.begin(), 1);
    std::cout << "Container for sorted unique vector" << std::endl;
    v4.print();

    Container<int, std::list<int>, And<Sorted, Unique>> l4;
    l4.push_back(6);
    l4.push_back(1);
    l4.push_back(1);
    l4.push_front(4);
    l4.insert(l4.begin(), 1);
    std::cout << "Container for sorted unique list" << std::endl;
    l4.print();

    Container<int, std::vector<int>, And<Unique, Sorted>> v5;
    v5.push_back(6);
    v5.push_back(1);
    v5.insert(v5.begin(), 1);
    std::cout << "Container for unique sorted vector" << std::endl;
    v5.print();

    Container<int, std::list<int>, And<Unique, Sorted>> l5;
    l5.push_back(6);
    l5.push_back(1);
    l5.push_back(1);
    l5.push_front(4);
    l5.insert(l5.begin(), 1);
    std::cout << "Container for unique sorted list" << std::endl;
    l5.print();

    return 0;
}