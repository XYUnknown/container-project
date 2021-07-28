#include <iostream>
#include <utility>
#include <typeinfo>
#include <type_traits>
#include <concepts>
#include <algorithm>
#include <functional>
#include <iterator>
#include <optional>

#include <list>
#include <set>
#include <map>
#include <unordered_set>
#include <unordered_map>

#include <stack>
#include <queue>

class Iterable {};

class LookUp {};

class Map {};

class Unique {};

class LIFO {};
class FIFO {};
template<class T, class CMP=std::less<T>>
class HeapOrd{};

template<class T, class CMP=std::less<T>, bool IsEager=true>
class Sorted {};

template<class T, class C, class... Ps>
class WithProperty: public C {
public:
    template<typename P>
    static constexpr bool has_property()
    { return std::disjunction_v<std::is_same<P, Ps>...>; }
};

template<class K, class V, class C, class... Ps>
class MapWithProperty: public C {
public:
    template<typename P>
    static constexpr bool has_property()
    { return std::disjunction_v<std::is_same<P, Ps>...>; }
};

template<class T>
using TreeSetWrapperAsc = WithProperty<T, std::set<T, std::less<T>>, Unique, Sorted<T, std::less<T>>>;

template<class T>
using TreeSetWrapperDesc = WithProperty<T, std::set<T, std::greater<T>>, Unique, Sorted<T, std::greater<T>>>;

template<class T>
using HashSetWrapper = WithProperty<T, std::unordered_set<T>, Unique>;

template<class K, class V>
using TreeMapWrapperAsc = MapWithProperty<K, V, std::map<K, V, std::less<K>>, Unique, Sorted<K, std::less<K>>>;

template<class K, class V>
using TreeMapWrapperDesc = MapWithProperty<K, V, std::map<K, V, std::greater<K>>, Unique, Sorted<K, std::greater<K>>>;

template<class K, class V>
using HashMapWrapper = MapWithProperty<K, V, std::unordered_map<K, V>, Unique>;

template<class C>
concept CUnique = C::template has_property<Unique>();

template<class T, class C>
concept CSorted = C::template has_property<Sorted<T>>();

template<class T, template<class...> class C, class... Ps>
class Container;

template<class P, class ...Ps>
constexpr bool is_present() {
    return (std::is_same_v<P, Ps> || ...);
}

// The minimal container interface
template<class T, template<class...> class C>
class Container<T, C> : protected C<T> {
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

public:
    // The number of elements of the underlying container
    using C<T>::size;
    // If the underlying container is empty
    using C<T>::empty;
    // Remove all elements from the underlying container
    using C<T>::clear;

    template <class Q = T>
    const Q& peek() const {
        // if a container has back
        if constexpr (requires (const C<Q>& c) { { c.back() } -> std::same_as<const Q&>; }){
            return C<Q>::back();
        } else { // if a container doesn't back
           return *(std::prev(C<Q>::end())); 
        }
    }

    template <class Q = T>
    void insert(Q t) { 
        // if a container has insert
        if constexpr (requires (C<Q>& c, Q val) { c.insert(val); }) {
            C<Q>::insert(t);
        } else {
            C<Q>::push_back(t);
        }
    }

    template <class Q = T>
    void pop() {
        // if a container has pop_back
        if constexpr (requires (C<Q>& c) { { c.pop_back() } -> std::same_as<void>; }) {
            C<Q>::pop_back();
        } else { // if a container does not have pop_back
            C<Q>::erase(std::prev(C<Q>::end()));
        }
    }
};

template<class K, class V, template<class...> class C>
class Container<std::pair<K, V>, C, Map> : protected C<K, V> {
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
public:
    using C<K, V>::size;
    using C<K, V>::empty;
    using C<K, V>::clear;
    using C<K, V>::insert;
    using C<K, V>::contains;
    using C<K, V>::at;

    void pop() {
        C<K, V>::erase(std::prev(C<K, V>::end()));
    }

    const std::pair<K, V>& peek() const {
        return *(std::prev(C<K, V>::end())); 
    }

    auto lookup(const K& key) {
        return (C<K, V>::find(key) != C<K, V>::end()) 
                ? std::optional<V>{C<K, V>::find(key)->second} 
                : std::nullopt;
    }
};

template<class T, template<class...> class C>
class Container<T, C, Sorted<T, LIFO>> : private std::stack<T, C<T>> {
public:
    using std::stack<T, C<T>>::size;
    using std::stack<T, C<T>>::empty;
    using std::stack<T, C<T>>::pop;

    void clear() {
        while(!this->empty()) 
            this->pop();
    }

    const T& peek() const {
        return std::stack<T, C<T>>::top();
    }

    void insert(const T& t) { 
        std::stack<T, C<T>>::push(t);
    }
};

template<class T, template<class...> class C>
class Container<T, C, Sorted<T, FIFO>> : private std::queue<T, C<T>> {
public:
    using std::queue<T, C<T>>::size;
    using std::queue<T, C<T>>::empty;
    using std::queue<T, C<T>>::pop;

    void clear() {
        while(!this->empty()) {
            this->pop();
        }
    }

    const T& peek() const {
        return std::queue<T, C<T>>::front();
    }

    void insert(const T& t) { 
        std::queue<T, C<T>>::push(t);
    }
};

template<class T, template<class...> class C, class CMP>
class Container<T, C, Sorted<T, HeapOrd<T, CMP>>> : private std::priority_queue<T, C<T>, CMP> {
public:
    using std::priority_queue<T, C<T>, CMP>::size;
    using std::priority_queue<T, C<T>, CMP>::empty;
    using std::priority_queue<T, C<T>, CMP>::pop;

    void clear() {
        while(!this->empty()) {
            this->pop();
        }
    }

    const T& peek() const {
        return std::priority_queue<T, C<T>, CMP>::top();
    }

    void insert(const T& t) { 
        std::priority_queue<T, C<T>, CMP>::push(t);
    }
};

template<class K, class V, template<class...> class C>
class Container<std::pair<K, V>, C, Iterable, Map> : public Container<std::pair<K, V>, C, Map> {
public:
    using C<K, V>::begin;
    using C<K, V>::end;
    using C<K, V>::erase;
    using C<K, V>::find;
};

template<class T, template<class...> class C, class ...Ps>
class Container<T, C, LookUp, Ps...> : public Container<T, C, Ps...> {
    friend constexpr auto operator<= (Container<T, C, LookUp, Ps...>const & lhs, Container<T, C, LookUp, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) <= static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator< (Container<T, C, LookUp, Ps...>const & lhs, Container<T, C, LookUp, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) < static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator== (Container<T, C, LookUp, Ps...>const & lhs, Container<T, C, LookUp, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) == static_cast<Container<T, C, Ps...>const &>(rhs));
    }

    friend constexpr auto operator!= (Container<T, C, LookUp, Ps...>const & lhs, Container<T, C, LookUp, Ps...>const & rhs) {
        return (static_cast<Container<T, C, Ps...>const &>(lhs) != static_cast<Container<T, C, Ps...>const &>(rhs));
    }
public:
    template <class Q = T>
    const Q& at(size_t pos) {
        if constexpr (requires (const C<Q>& c, size_t p) { { c.at(p) } -> std::same_as<const Q&>; }) {
            return C<Q>::at(pos);
        } else {
            auto iter = C<Q>::begin();
            for (size_t i=0; i<pos; i++) {
                iter++;
            }
            return *iter;
        }
    }

    template <class Q = T>
    bool contains(const Q& value) {
        if constexpr (requires (C<Q>& c, const Q& t) { { c.contains(t) } -> std::same_as<bool>; }) {
            return C<Q>::contains(value);
        } else {
            return this->find(value) != C<Q>::end();
        }
    }
};

// A iterable container interface
template<class T, template<class...> class C>
class Container<T, C, Iterable> : public Container<T, C> {
    friend constexpr auto operator<= (Container<T, C, Iterable>const & lhs, Container<T, C, Iterable>const & rhs) {
        return (static_cast<Container<T, C>const &>(lhs) <= static_cast<Container<T, C>const &>(rhs));
    }

    friend constexpr auto operator< (Container<T, C, Iterable>const & lhs, Container<T, C, Iterable>const & rhs) {
        return (static_cast<Container<T, C>const &>(lhs) < static_cast<Container<T, C>const &>(rhs));
    }

    friend constexpr auto operator== (Container<T, C, Iterable>const & lhs, Container<T, C, Iterable>const & rhs) {
        return (static_cast<Container<T, C>const &>(lhs) == static_cast<Container<T, C>const &>(rhs));
    }

    friend constexpr auto operator!= (Container<T, C, Iterable>const & lhs, Container<T, C, Iterable>const & rhs) {
        return (static_cast<Container<T, C>const &>(lhs) != static_cast<Container<T, C>const &>(rhs));
    }

public:
    using Container<T, C>::peek;
    using Container<T, C>::insert;
    using Container<T, C>::pop;
    using Container<T, C>::size;
    using Container<T, C>::empty;
    using Container<T, C>::clear;

    using C<T>::begin;
    using C<T>::end;
    using C<T>::erase;
    using C<T>::emplace;

    typename C<T>::iterator insert(typename C<T>::iterator pos, const T& t) {
        return C<T>::insert(pos, t);
    }

    template <class Q = T>
    typename C<Q>::iterator find(const Q& value) {
        if constexpr (requires (C<Q>& c, const Q& t) { { c.find(t) } -> std::same_as<typename C<Q>::iterator>; }) {
            return C<Q>::find(value);
        } else {
            return std::find(this->begin(), this->end(), value);
        } 
    }
};

// Unique Property
template<class T, template<typename...> class C, class ...Ps>
class Container<T, C, Unique, Ps...> : private Container<T, C, Ps...> {
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
public:
    using Container<T, C, Ps...>::size;
    using Container<T, C, Ps...>::empty;

    using Container<T, C, Ps...>::peek;
    using Container<T, C, Ps...>::begin;
    using Container<T, C, Ps...>::end;

    using Container<T, C, Ps...>::pop;
    using Container<T, C, Ps...>::erase;
    using Container<T, C, Ps...>::clear;

    using Container<T, C, Ps...>::find;
    using Container<T, C, Ps...>::contains;
    using Container<T, C, Ps...>::at;

    void insert(typename C<T>::iterator pos, T t) {
        if constexpr (CUnique<C<T>>) {
            std::cout<<"called"<<std::endl;
            Container<T, C, Ps...>::insert(pos, t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::insert(pos, t);
            }
        }
    }

    void insert(T t) {
        if constexpr (CUnique<C<T>>) {
            Container<T, C, Ps...>::insert(t);
        } else {
            if (!this->contains(t)) {
                Container<T, C, Ps...>::insert(t);
            }
        }
    }
};

// Sorted Property -- eager
template<class T, template<typename...> class C, class CMP, class ...Ps>
class Container<T, C, Sorted<T, CMP>, Ps...> : private Container<T, C, Ps...> {
public:
    using Container<T, C, Ps...>::size;
    using Container<T, C, Ps...>::empty;

    using Container<T, C, Ps...>::peek;
    using Container<T, C, Ps...>::begin;
    using Container<T, C, Ps...>::end;

    using Container<T, C, Ps...>::pop;
    using Container<T, C, Ps...>::erase;
    using Container<T, C, Ps...>::clear;
    using Container<T, C, Ps...>::at;

    // insert(pos, t) is removed
    void insert(T t) {
        if constexpr (CSorted<T, C<T>>) {
            Container<T, C, Ps...>::insert(t);
        } else {
            auto pos = std::lower_bound(this->begin(), this->end(), t, CMP());
            Container<T, C, Ps...>::insert(pos, t);
        }
    }

    bool contains(const T& t) {
        if constexpr (CSorted<T, C<T>>){
            return Container<T, C, Ps...>::contains(t);
        } else {
            return std::binary_search(this->begin(), this->end(), t, CMP());
        }
    }

    typename C<T>::iterator find(const T& t) {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::find(t);
        } else {
            auto pos = std::lower_bound(this->begin(), this->end(), t, CMP());
            if (*pos != t) { // element not found
                return this->end();
            }
            return pos;
        }
    }
};

// Sorted Property -- lazy
template<class T, template<typename...> class C, class CMP, class ...Ps>
class Container<T, C, Sorted<T, CMP, false>, Ps...> : private Container<T, C, Ps...> {
private:
    template <class Q = T>
    requires requires (C<Q>& c) { { c.sort() } -> std::same_as<void>; }
    void sort() {
        return C<Q>::sort();
    }

    template <class Q = T, class CMPs = CMP>
        requires requires (C<Q>& c, CMPs cmp) { { c.sort(cmp) } -> std::same_as<void>; }
    void sort(CMPs cmp) {
        return C<Q>::sort(cmp);
    }

    template <class Q = T>
        requires (!requires (C<Q>& c) { { c.sort() } -> std::same_as<void>; })
    void sort() {
        std::sort(Container<T, C, Ps...>::begin(), Container<T, C, Ps...>::end());
    }

    template <class Q = T, class CMPs = CMP>
        requires (!requires (C<Q>& c, CMPs cmp) { { c.sort(cmp) } -> std::same_as<void>; })
    void sort(CMPs cmp) {
        std::sort(Container<T, C, Ps...>::begin(), Container<T, C, Ps...>::end(), cmp);
    }

    bool is_sorted = true;
    void sort_on_access() {
        this->sort(CMP());
        is_sorted = true;
    }

public:
    using Container<T, C, Ps...>::size;
    using Container<T, C, Ps...>::empty;
    using Container<T, C, Ps...>::clear;
    using Container<T, C, Ps...>::at;

    // insert(pos, t) is removed
    void insert(T t) {
        if constexpr (CSorted<T, C<T>>) {
            Container<T, C, Ps...>::insert(t);
        } else {
            this->is_sorted = false;
            Container<T, C, Ps...>::insert(t);
        }
    }

    auto begin() {
        if constexpr (CSorted<T, C<T>>){
            return Container<T, C, Ps...>::begin();
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return Container<T, C, Ps...>::begin();
        }
    }

    auto end() {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::end();
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return Container<T, C, Ps...>::end();
        }
    }

    auto peek() {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::peek();
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return Container<T, C, Ps...>::peek();
        }
    }

    bool contains(const T& t) {
        if constexpr (CSorted<T, C<T>>){
            return Container<T, C, Ps...>::contains(t);
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return std::binary_search(this->begin(), this->end(), t, CMP());
        }
    }

    typename C<T>::iterator find(const T& t) {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::find(t);
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            auto pos = std::lower_bound(this->begin(), this->end(), t, CMP());
            if (*pos != t) { // element not found
                return this->end();
            }
            return pos;
        }
    }

    auto at(size_t pos) {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::at(pos);
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return Container<T, C, Ps...>::at(pos);
        }
    }

    void pop() {
        if constexpr (CSorted<T, C<T>>){
            Container<T, C, Ps...>::pop();
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            Container<T, C, Ps...>::pop();
        }
    }

    auto erase(typename C<T>::iterator pos) {
        if constexpr (CSorted<T, C<T>>) {
            return Container<T, C, Ps...>::erase(pos);
        } else {
            if (!this->is_sorted) {
                this->sort_on_access();
            }
            return Container<T, C, Ps...>::erase(pos);
        }
    }
};