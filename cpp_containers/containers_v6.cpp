#include <iostream>
#include <utility>
#include <vector>
#include <list>
#include <set>
#include <typeinfo>
#include <type_traits>
#include <concepts>

class Unique {};
class Sorted {};

template<class T, template<class...> class C, class... Ps>
struct Container;

template<class T, template<class...> class C, class... Ps>
struct WithProperty : public C<T> {
    template<typename P>
    static constexpr bool has_property()
    { return std::disjunction_v<std::is_same<P, Ps>...>; }
};

// A set implemented using a binary search tree
template<class T>
using SetWrapper = WithProperty<T, std::set, Unique, Sorted>;


// a binary search tree
template<class T>
using TreeWrapper = WithProperty<T, std::multiset, Sorted>;

template<typename T, template<class...> class C>
concept CUnique = C<T>::template has_property<Unique>();

template<typename T, template<class...> class C>
concept CSorted = C<T>::template has_property<Sorted>();

template<class T, template<class...> class C>
struct Container<T, C> : private C<T> {
    friend constexpr auto operator<= (const Container<T, C>& lhs, const Container<T, C>& rhs) {
        return (static_cast<const C<T> &>(lhs) <= static_cast<const C<T> &>(rhs));
    }

    friend constexpr auto operator< (const Container<T, C>& lhs, const Container<T, C>& rhs) {
        return (static_cast<const C<T> &>(lhs) < static_cast<const C<T> &>(rhs));
    }
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
    typename C<Q>::iterator insert(typename C<Q>::iterator pos, Q t) {
        return C<Q>::insert(pos, t);
    }

    // if a container has insert(val)
    template <class Q = T>
        requires (requires (C<Q>& c, Q val) { { c.insert(val) } -> std::same_as<std::pair<typename C<Q>::iterator, bool>>; })
                || (requires (C<Q>& c, Q val) { { c.insert(val) } -> std::same_as<typename C<Q>::iterator>; })
    auto insert(Q t) {
        return C<Q>::insert(t);
    }

    // if a container does not have insert val
    template <class Q = T>
        requires (!requires (C<Q>& c, Q val) { { c.insert(val) } -> std::same_as<std::pair<typename C<Q>::iterator, bool>>; })
                && (!requires (C<Q>& c, Q val) { { c.insert(val) } -> std::same_as<typename C<Q>::iterator>; })
    std::pair<typename C<Q>::iterator, bool> insert(Q t) {
        size_t old_size = this->size();
        C<Q>::push_back(t);
        return std::pair(this->end(), (old_size < this->size()));
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

    template <class Q = T>
        requires requires (C<Q>& c, typename C<Q>::iterator pos) { { c.erase(pos) } -> std::same_as<typename C<Q>::iterator>; }
    typename C<Q>::iterator erase(typename C<Q>::iterator pos) {
        return C<Q>::erase(pos);
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
struct Container<T, C, Unique, Ps...> : private Container<T, C, Ps...> {
    using Container<T, C, Ps...>::begin;
    using Container<T, C, Ps...>::end;
    using Container<T, C, Ps...>::size;
    using Container<T, C, Ps...>::empty;
    using Container<T, C, Ps...>::contains;
    using Container<T, C, Ps...>::print;

    friend constexpr auto operator<= (const Container<T, C, Unique, Ps...>& lhs, const Container<T, C, Unique, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) <= static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    friend constexpr auto operator< (const Container<T, C, Unique, Ps...>& lhs, const Container<T, C, Unique, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) < static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    void push_back(T t) {
        if constexpr (CUnique<T, C>) {
            std::cout << "CUnique specialization is called" << std::endl;
            Container<T, C, Ps...>::push_back(t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::push_back(t);
            }
        }
    }

    void push_front(T t) {
        if constexpr (CUnique<T, C>) {
            std::cout << "CUnique specialization is called" << std::endl;
            Container<T, C, Ps...>::push_front(t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::push_front(t);
            }
        }
    }

    auto insert(typename C<T>::iterator pos, T t) {
        if constexpr (CUnique<T, C>) {
            std::cout << "CUnique specialization is called" << std::endl;
            Container<T, C, Ps...>::insert(pos, t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::insert(pos, t);
            }
        }
    }

    auto insert(T t) {
        if constexpr (CUnique<T, C>) {
            std::cout << "CUnique specialization is called" << std::endl;
            Container<T, C, Ps...>::insert(t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::insert(t);
            }
        }
    }
};

template<class T, template<typename...> class C, class ...Ps>
struct Container<T, C, Sorted, Ps...> : private Container<T, C, Ps...> {
    using Container<T, C, Ps...>::begin;
    using Container<T, C, Ps...>::end;
    using Container<T, C, Ps...>::size;
    using Container<T, C, Ps...>::empty;
    using Container<T, C, Ps...>::contains;
    using Container<T, C, Ps...>::print;

    friend constexpr auto operator<= (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...> &>(lhs) <= static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    friend constexpr auto operator< (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) < static_cast<const Container<T, C, Ps...>&>(rhs));
    }
    // for a sorted container, it is not meaningful to choose the position for an element
    // to be inserted in.
    auto insert(T t) {
        if constexpr (CSorted<T, C>) {
            std::cout << "CSorted specialization is called" << std::endl;
            Container<T, C, Ps...>::insert(t);
        } else {
            auto pos = std::lower_bound(this->begin(), this->end(), t);
            Container<T, C, Ps...>::insert(pos, t);
        }
    }
};

template<class P, class ...Ps>
constexpr bool is_present() {
    return (std::is_same_v<P, Ps> || ...);
}

template<class T, class ...Ps>
auto make_container() {
    if constexpr (is_present<Sorted, Ps...>()) {
        if constexpr (is_present<Unique, Ps...>()) {
            Container<T, std::set, Ps...> c;
            return c;
        } else {
            Container<T, std::multiset, Ps...> c;
            return c;
        }
    } else {
        Container<T, std::vector, Ps...> c;
        return c;
    }
}

int main() {
    Container<int, std::vector> v1;
    v1.insert(6);
    v1.push_back(3);
    v1.insert(v1.begin(), 5);
    v1.insert(v1.end(), 7);
    v1.erase(v1.begin());
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
    l1.erase(l1.begin());
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
    v3.insert(6);
    v3.insert(1);
    v3.insert(6);
    v3.insert(5);
    std::cout << "Container for sorted vector" << std::endl;
    v3.print();

    Container<int, std::list, Sorted> l3;
    l3.insert(6);
    l3.insert(4);
    l3.insert(1);
    l3.insert(4);
    std::cout << "Container for sorted list" << std::endl;
    l3.print();

    Container<int, std::vector, Sorted, Unique> v4;
    v4.insert(6);
    v4.insert(1);
    v4.insert(1);
    std::cout << "Container for sorted unique vector" << std::endl;
    v4.print();

    Container<int, std::list, Sorted, Unique> l4;
    l4.insert(6);
    l4.insert(1);
    l4.insert(1);
    std::cout << "Container for sorted unique list" << std::endl;
    l4.print();

    Container<int, std::vector, Unique, Sorted> v5;
    v5.insert(6);
    v5.insert(1);
    v5.insert(1);
    std::cout << "Container for unique sorted vector" << std::endl;
    v5.print();

    Container<int, std::list, Unique, Sorted> l5;
    l5.insert(6);
    l5.insert(1);
    l5.insert(1);
    std::cout << "Container for unique sorted list" << std::endl;
    l5.print();

    SetWrapper<int> sw;
    sw.size();
    static_assert(sw.has_property<Unique>()); // not recommended to call in this way

    Container<int, SetWrapper> s1;
    s1.insert(6);
    s1.insert(1);
    s1.insert(1);
    std::cout << "Container for set(tree set) is sorted and unique" << std::endl;
    s1.print();

    Container<int, SetWrapper, Unique> s2;
    s2.insert(6);
    s2.insert(1);
    s2.insert(1);
    std::cout << "Container for unique set(tree set) is sorted and unique" << std::endl;
    s2.print();

    Container<int, SetWrapper, Sorted> s3;
    s3.insert(6);
    s3.insert(1);
    s3.insert(1);
    std::cout << "Container for sorted set(tree set) is sorted and unique" << std::endl;
    s3.print();

    Container<int, SetWrapper, Unique, Sorted> s4;
    s4.insert(6);
    s4.insert(1);
    s4.insert(1);
    std::cout << "Container for unique and sorted set(tree set) is sorted and unique" << std::endl;
    s4.print();

    TreeWrapper<int> tw;
    tw.size();
    static_assert(tw.has_property<Sorted>());
    //static_assert(tw.has_property<Unique>());//fail

    static_assert(CSorted<int, TreeWrapper>);
    //static_assert(CUnique<int, TreeWrapper>);

    Container<int, TreeWrapper> t1;
    t1.insert(6);
    t1.insert(1);
    t1.insert(1);
    std::cout << "Container for tree (multiset) is sorted" << std::endl;
    t1.print();

    Container<int, TreeWrapper, Unique> t2;
    t2.insert(6);
    t2.insert(1);
    t2.insert(1);
    std::cout << "Container for unique tree (multiset) is unique sorted" << std::endl;
    t2.print();

    Container<int, TreeWrapper, Sorted> t3;
    t3.insert(6);
    t3.insert(1);
    t3.insert(1);
    std::cout << "Container for sorted tree (multiset) is sorted" << std::endl;
    t3.print();

    Container<int, TreeWrapper, Unique, Sorted> t4;
    t4.insert(6);
    t4.insert(1);
    t4.insert(1);
    std::cout << "Container for sorted tree (multiset) is sorted" << std::endl;
    t4.print();

    // Nested containers
    Container<Container<int, std::vector, Unique>, std::vector, Sorted> n1;
    Container<int, std::vector, Unique> e1, e2, e3;
    e1.insert(4);
    e1.insert(3);
    e2.insert(5);
    e2.insert(1);
    e2.insert(10);
    e3.insert(11);
    n1.insert(e1);
    n1.insert(e2);
    n1.insert(e3);
    std::cout << "Size: " << "Nested container" << std::endl;
    std::cout << "Size: " << n1.size() << std::endl;
    for (auto it=n1.begin(); it!=n1.end(); it++)
        it->print();
    std::cout << '\n';
    std::cout << '\n';
    
    // test is present
    static_assert(!is_present<Unique, Sorted>());
    static_assert(is_present<Unique, Unique, Sorted>());

    // test the make_container
    auto c1 = make_container<int, Unique>();
    c1.insert(6);
    c1.insert(1);
    c1.insert(1);
    std::cout << "Unique Container from function make_container" << std::endl;
    c1.print();

    auto c2 = make_container<int, Sorted>();
    c2.insert(6);
    c2.insert(1);
    c2.insert(1);
    std::cout << "Sorted Container from function make_container" << std::endl;
    c2.print();

    auto c3 = make_container<int, Unique, Sorted>();
    c3.insert(6);
    c3.insert(1);
    c3.insert(1);
    std::cout << "Unique Sorted Container from function make_container" << std::endl;
    c3.print();

    return 0;
}