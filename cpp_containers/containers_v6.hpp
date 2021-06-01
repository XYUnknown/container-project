#include <iostream>
#include <utility>
#include <vector>
#include <list>
#include <set>
#include <map>
#include <unordered_set>
#include <typeinfo>
#include <type_traits>
#include <concepts>
#include <algorithm>

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
using TreeSetWrapper = WithProperty<T, std::set, Unique, Sorted>;

// a binary search tree
template<class T>
using TreeWrapper = WithProperty<T, std::multiset, Sorted>;

// a hashset wrapper
template<class T>
using HashSetWrapper = WithProperty<T, std::unordered_set, Unique>;

template<typename T, template<class...> class C>
concept CUnique = C<T>::template has_property<Unique>();

template<typename T, template<class...> class C>
concept CSorted = C<T>::template has_property<Sorted>();

template<class T, template<class...> class C>
struct Container<T, C> : private C<T> {
    friend constexpr auto operator<= (Container<T, C>const & lhs, Container<T, C>const & rhs) {
        return (static_cast<C<T>const &>(lhs) <= static_cast<C<T>const &>(rhs));
    }

    friend constexpr auto operator< (Container<T, C>const & lhs, Container<T, C>const & rhs) {
        return (static_cast<C<T>const &>(lhs) < static_cast<C<T>const &>(rhs));
    }

    friend constexpr auto operator== (Container<T, C>const & lhs, Container<T, C>const & rhs) {
        return (static_cast<C<T>const &>(lhs) == static_cast<C<T>const &>(rhs));
    }

    friend constexpr auto operator!= (Container<T, C>const & lhs, Container<T, C>const & rhs) {
        return (static_cast<C<T>const &>(lhs) != static_cast<C<T>const &>(rhs));
    }

    // The first requires is the require claus, the second requires is the require expression
    template <class Q = T>
        requires requires (const C<Q>& c, size_t p) { { c.at(p) } -> std::same_as<const Q&>; }
    Q& at(size_t pos) {
        return C<Q>::at(pos);
    }

    template <class Q = T>
        requires requires (C<Q>& c, size_t new_cap) { { c.reserve(new_cap) } -> std::same_as<void>; }
    void reserve(size_t new_cap) {
        return C<Q>::reserve(new_cap);
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

    template <class Q = T>
        requires requires (C<Q>& c, const Q& t) { { c.find(t) } -> std::same_as<typename C<Q>::iterator>; }
    typename C<Q>::iterator find(const Q& value) {
        return C<Q>::find(value);
    }

    template <class Q = T>
        requires (!requires (C<Q>& c, const Q& t) { { c.find(t) } -> std::same_as<typename C<Q>::iterator>; })
    typename C<Q>::iterator find(const Q& value) {
        return std::find(this->begin(), this->end(), value);
    }

    template <class Q = T>
        requires requires (C<Q>& c, const Q& t) { { c.contains(t) } -> std::same_as<bool>; }
    bool contains(const Q& value) /*const*/ {
        return C<Q>::contains(value);
    }

    template <class Q = T>
        requires (!requires (C<Q>& c, const Q& t) { { c.contains(t) } -> std::same_as<bool>; })
    bool contains(const Q& value) {
        return this->find(value) != this->end();
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
    using Container<T, C, Ps...>::find;
    using Container<T, C, Ps...>::clear;
    using Container<T, C, Ps...>::erase;
    using Container<T, C, Ps...>::at;
    using Container<T, C, Ps...>::reserve;

    friend constexpr auto operator<= (Container<T, C, Unique, Ps...>const & lhs, Container<T, C, Unique, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) <= static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator< (Container<T, C, Unique, Ps...>const & lhs, Container<T, C, Unique, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) < static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator== (Container<T, C, Unique, Ps...>const & lhs, Container<T, C, Unique, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) == static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator!= (Container<T, C, Unique, Ps...>const & lhs, Container<T, C, Unique, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) != static_cast<Container<T, C, Ps...>const &>(rhs));
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
    using Container<T, C, Ps...>::print;
    using Container<T, C, Ps...>::clear;
    using Container<T, C, Ps...>::erase;
    using Container<T, C, Ps...>::at;
    using Container<T, C, Ps...>::reserve;


    friend constexpr auto operator<= (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...> &>(lhs) <= static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    friend constexpr auto operator< (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) < static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    friend constexpr auto operator== (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) == static_cast<const Container<T, C, Ps...>&>(rhs));
    }

    friend constexpr auto operator!= (const Container<T, C, Sorted, Ps...>& lhs, const Container<T, C, Sorted, Ps...>& rhs) {
        return (static_cast<const Container<T, C, Ps...>&>(lhs) != static_cast<const Container<T, C, Ps...>&>(rhs));
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

    bool contains(const T& t) {
        if constexpr (CSorted<T, C>) {
            std::cout << "CSorted specialization is called" << std::endl;
            return Container<T, C, Ps...>::contains(t);
        } else {
            return std::binary_search(this->begin(), this->end(), t);
        }
    }

    typename C<T>::iterator find(const T& t) {
        if constexpr (CSorted<T, C>) {
            std::cout << "CSorted specialization is called" << std::endl;
            return Container<T, C, Ps...>::find(t);
        } else {
            auto pos = std::lower_bound(this->begin(), this->end(), t);
            if (*pos != t) { // element not found
                return this->end();
            }
            return pos;
        }
    }
};

/* Maps */
template<class K, class V, template<class...> class C>
struct Container<std::pair<K, V>, C> : private C<K, V> {
    friend constexpr auto operator<= (Container<std::pair<K, V>, C>const & lhs, Container<std::pair<K, V>, C>const & rhs) {
        return (static_cast<C<K, V>const &>(lhs) <= static_cast<C<K, V>const &>(rhs));
    }

    friend constexpr auto operator< (Container<std::pair<K, V>, C>const & lhs, Container<std::pair<K, V>, C>const & rhs) {
        return (static_cast<C<K, V>const &>(lhs) < static_cast<C<K, V>const &>(rhs));
    }

    friend constexpr auto operator== (Container<std::pair<K, V>, C>const & lhs, Container<std::pair<K, V>, C>const & rhs) {
        return (static_cast<C<K, V>const &>(lhs) == static_cast<C<K, V>const &>(rhs));
    }

    friend constexpr auto operator!= (Container<std::pair<K, V>, C>const & lhs, Container<std::pair<K, V>, C>const & rhs) {
        return (static_cast<C<K, V>const &>(lhs) != static_cast<C<K, V>const &>(rhs));
    }

    template <class K1 = K, class V1 = V>
        requires requires (const C<K1, V1>& c, K1 key) { { c.at(key) } -> std::same_as<V1&>; }
    V1& at(const K1& key) {
        return C<K1, V1>::at(key);
    }

    template <class K1 = K, class V1 = V>
        requires requires (const C<K1, V1>& c) { { c.size() } -> std::same_as<size_t>; }
    size_t size() {
        return C<K1, V1>::size();
    }

    template <class K1 = K, class V1 = V>
        requires requires (const C<K1, V1>& c) { { c.empty() } -> std::same_as<bool>; }
    bool empty() {
        return C<K1, V1>::empty();
    }

    template <class K1 = K, class V1 = V>
        requires requires (C<K1, V1>& c) { { c.clear() } -> std::same_as<void>; }
    void clear() {
        C<K1, V1>::clear();
    }

    template <class K1 = K, class V1 = V>
        requires requires (C<K1, V1>& c) { { c.begin() } -> std::same_as<typename C<K1, V1>::iterator>; }
    typename C<K1, V1>::iterator begin() {
        return C<K1, V1>::begin();
    }

    template <class K1 = K, class V1 = V>
        requires requires (C<K1, V1>& c) { { c.end() } -> std::same_as<typename C<K1, V1>::iterator>; }
    typename C<K1, V1>::iterator end() {
        return C<K1, V1>::end();
    }

    void print() {
        std::cout << "Size: " << this->size() << std::endl;
        for (auto it=this->begin(); it!=this->end(); it++)
            std::cout << it->first << ", " << it->second << '\n';
        std::cout << '\n';
        std::cout << '\n';
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
            Container<T, TreeSetWrapper, Ps...> c;
            return c;
        } else {
            Container<T, TreeWrapper, Ps...> c;
            return c;
        }
    } else if constexpr (is_present<Unique, Ps...>()) {
        if constexpr (is_present<Sorted, Ps...>()) {
            Container<T, TreeSetWrapper, Ps...> c;
            return c;
        } else {
            Container<T, HashSetWrapper, Ps...> c;
            return c;
        }
    } else {
        Container<T, std::vector, Ps...> c;
        return c;
    }
}
