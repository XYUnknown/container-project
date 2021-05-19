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
    using Container<T, C, Ps...>::contains;
    using Container<T, C, Ps...>::print;

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
