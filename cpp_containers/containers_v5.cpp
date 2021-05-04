#include <iostream>
#include <vector>
#include <list>
#include <set>
#include <typeinfo>
#include <type_traits>
#include <concepts>

class Unique {};
class Sorted {};

template<class T, template<class...> class C, class... P>
struct Container;

template<class T> struct dependent_false : std::false_type {};

template<class T, template<class...> class C>
struct Container<T, C> : private C<T> {
    // The first requires is the require claus, the second requires is the require expression
    template <class Q = T>
        requires requires (const C<Q>& c, size_t p) { { c.at(p) } -> std::same_as<const Q&>; }
    Q& at(size_t pos) {
        return C<Q>::at(pos);
    }

    template <class Q = T>
        requires requires (const C<Q>& c) { { c.size() } -> std::same_as<size_t>; }
    size_t size() {
        return C<Q>::size();
    }

    template <class Q = T>
        requires requires (const C<Q>& c) { { c.empty() } -> std::same_as<bool>; }
    bool empty() {
        return C<Q>::empty();
    }

    template <class Q = T>
        requires requires (C<Q>& c) { { c.clear() } -> std::same_as<void>; }
    void clear() {
        C<Q>::clear();
    }

    template <class Q = T>
        requires requires (C<Q>& c, Q val) { { c.push_back(val) } -> std::same_as<void>; }
    void push_back(Q t) {
        C<Q>::push_back(t);
    }

    template <class Q = T>
        requires requires (C<Q>& c, Q val) { { c.push_front(val) } -> std::same_as<void>; }
    void push_front(Q t) {
        C<Q>::push_front(t);
    }

    template <class Q = T>
        requires requires (C<Q>& c, typename C<Q>::iterator pos, Q val) { { c.insert(pos, val) } -> std::same_as<typename C<Q>::iterator>; }
    typename C<Q>::iterator insert(typename C<Q>::iterator pos, T t) {
        return C<Q>::insert(pos, t);
    }

    template <class Q = T>
        requires requires (C<Q>& c) { { c.begin() } -> std::same_as<typename C<Q>::iterator>; }
    typename C<Q>::iterator begin() {
        return C<Q>::begin();
    }

    template <class Q = T>
        requires requires (C<Q>& c) { { c.end() } -> std::same_as<typename C<Q>::iterator>; }
    typename C<Q>::iterator end() {
        return C<Q>::end();
    }

    bool contains(T t) {
        return std::find(this->begin(), this->end(), t) != this->end();
    }

    void print() {
        std::cout << "Size: " << this->size() << std::endl;
        for (auto it=this->begin(); it!=this->end(); it++)
            std::cout << ' ' << *it;
        std::cout << '\n';
        std::cout << '\n';
    }
};

template<class T, template<typename...> class C, class ...Ps>
struct Container<T, C, Unique, Ps...> : Container<T, C, Ps...> {
    auto push_back(T t) {
        if (!this->contains(t)) {
            Container<T, C, Ps...>::push_back(t);
        }
    }

    auto push_front(T t) {
        if (!this->contains(t)) {
            Container<T, C, Ps...>::push_front(t);
        }
    }

    auto insert(typename C<T>::iterator pos, T t) {
        if (!this->contains(t)) {
            Container<T, C, Ps...>::insert(pos, t);
        }
    }
};

template<class T, template<typename...> class C, class ...Ps>
struct Container<T, C, Sorted, Ps...> : Container<T, C, Ps...> {
    auto push_back(T t) {
        this->insert(this->end(), t);
    }

    auto push_front(T t) {
        if constexpr (std::is_same<C<T>, std::list<T>>::value) {
            this->insert(this->begin(), t);
        } else {
            static_assert(dependent_false<T>::value, "push_front is only defined for list");
        }
    }

    auto insert(typename C<T>::iterator pos, T t) {
        auto pos_i = std::lower_bound(this->begin(), pos, t);
        if (pos_i == pos) {
            pos_i = std::lower_bound(pos, this->end(), t);
            Container<T, C, Ps...>::insert(pos_i, t);
        } else {
            Container<T, C, Ps...>::insert(pos_i, t);
        }
    }
};

int main() {
    Container<int, std::vector> v1;
    v1.push_back(3);
    v1.insert(v1.begin(), 5);
    v1.insert(v1.end(), 7);
    std::cout << typeid(v1).name() << std::endl;
    std::cout << "at position 0: " << v1.at(0) << std::endl;
    std::cout << "is empty? " << v1.empty() << std::endl;
    std::cout << "Container for default vector" << std::endl;
    v1.print();
    v1.clear();
    std::cout << "After clear" << std::endl;
    v1.print();

    Container<int, std::list> l1;
    l1.push_back(3);
    l1.push_front(6);
    l1.insert(l1.begin(), 5);
    l1.insert(l1.end(), 7);
    // std::cout << l1.at(0) << std::endl; // error
    std::cout << typeid(l1).name() << std::endl;
    std::cout << "Container for default list" << std::endl;
    l1.print();
    std::cout << "is empty? " << l1.empty() << std::endl;
    l1.clear();
    std::cout << "After clear" << std::endl;
    l1.print();

    Container<int, std::vector, Unique> v2;
    v2.push_back(1);
    v2.push_back(1);
    // v2.push_front(1); // invalid
    v2.insert(v2.begin(), 1);
    std::cout << "Container for unique vector" << std::endl;
    v2.print();

    Container<int, std::list, Unique> l2;
    l2.push_back(1);
    l2.push_back(1);
    l2.push_front(1);
    l2.insert(l2.begin(), 1);
    std::cout << "Container for unique list" << std::endl;
    l2.print();

    Container<int, std::vector, Sorted> v3;
    v3.push_back(6);
    v3.push_back(1);
    v3.insert(v3.begin(), 1);
    std::cout << "Container for sorted vector" << std::endl;
    v3.print();

    Container<int, std::list, Sorted> l3;
    l3.push_back(6);
    l3.push_back(1);
    l3.push_front(4);
    l3.insert(l3.begin(), 1);
    std::cout << "Container for sorted list" << std::endl;
    l3.print();

    Container<int, std::vector, Sorted, Unique> v4;
    v4.push_back(6);
    v4.push_back(1);
    v4.insert(v4.begin(), 1);
    std::cout << "Container for sorted unique vector" << std::endl;
    v4.print();

    Container<int, std::list, Sorted, Unique> l4;
    l4.push_back(6);
    l4.push_back(1);
    l4.push_back(1);
    l4.push_front(4);
    l4.insert(l4.begin(), 1);
    std::cout << "Container for sorted unique list" << std::endl;
    l4.print();

    Container<int, std::vector, Unique, Sorted> v5;
    v5.push_back(6);
    v5.push_back(1);
    v5.insert(v5.begin(), 1);
    std::cout << "Container for unique sorted vector" << std::endl;
    v5.print();

    Container<int, std::list, Unique, Sorted> l5;
    l5.push_back(6);
    l5.push_back(1);
    l5.push_back(1);
    l5.push_front(4);
    l5.insert(l5.begin(), 1);
    std::cout << "Container for unique sorted list" << std::endl;
    l5.print();

    Container<int, std::set> s;
    s.size();
    //s.push_front(6);
    //s.insert(6);

    return 0;
}