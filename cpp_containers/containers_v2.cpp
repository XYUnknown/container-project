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
struct Container<T, C, void> : std::vector<T>, std::list<T> {
    auto at(size_t pos) {
        if constexpr (std::is_same<C, std::vector<T>>::value) {
            return std::vector<T>::at(pos);
        } else if constexpr (std::is_same<C, std::list<T>>::value) {
            static_assert(dependent_false<T>::value, "at is only defined for vectors");
        } else {
            // at compilation level
            // ref : https://en.cppreference.com/w/cpp/language/if
            static_assert(dependent_false<T>::value, "at is only defined for vectors");
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
            static_assert(dependent_false<T>::value, "push_front is only defined for list");
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

void print_vector(std::vector<int> v) {
    std::cout << "Size: " << v.size() << std::endl;
    for (auto it=v.begin(); it<v.end(); it++)
        std::cout << ' ' << *it;
    std::cout << '\n';
    std::cout << '\n';
}

void print_list(std::list<int> l) {
    std::cout << "Size: " << l.size() << std::endl;
    for (auto const &i : l)
        std::cout << ' ' << i;
    std::cout << '\n';
    std::cout << '\n';
}

int main() {
    // std::cout << std::is_same<std::vector<int>, std::vector<int>>::value << ' ';
    Container<int, std::vector<int>, void> v1;
    v1.push_back(3);
    v1.insert(v1.begin(), 5);
    std::cout << typeid(v1).name() << std::endl;
    std::cout << "Container for default vector" << std::endl;
    print_vector(v1);

    Container<int, std::list<int>, void> l1;
    l1.push_back(3);
    l1.push_front(6);
    l1.insert(l1.begin(), 5);
    std::cout << typeid(l1).name() << std::endl;
    std::cout << "Container for default list" << std::endl;
    print_list(l1);

    Container<int, std::vector<int>, Unique> v2;
    v2.push_back(1);
    v2.push_back(1);
    // v2.push_front(1); // invalid
    v2.insert(v2.begin(), 1);
    std::cout << "Container for unique vector" << std::endl;
    print_vector(v2);
    return 0;
}