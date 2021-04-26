#include <iostream>
#include <vector>
#include <list>
#include <typeinfo>
#include <type_traits>

// ref: https://stackoverflow.com/a/59161334
//      https://en.cppreference.com/w/cpp/types/is_invocable
#define RETURNS( ... ) \
  noexcept(noexcept(__VA_ARGS__)) \
  -> decltype(__VA_ARGS__) \
  { return __VA_ARGS__; }

#define METHOD( ... )  \
    []( auto&& self, auto&&...args ) \
    RETURNS( decltype(self)(self).__VA_ARGS__( decltype(args)(args)... ) )

class Unique {};
class Sorted {};

template<class T, template<class...> class C, class... P>
struct Container;

template<class T> struct dependent_false : std::false_type {};

template<class T, template<class...> class C>
struct Container<T, C> : private C<T> {
    T at(size_t pos) {
        auto call_size = METHOD(at);
        if constexpr (std::is_invocable_r_v<T, decltype(call_size), C<T>&, size_t>) {
            return C<T>::at(pos);
        } else {
            // at compilation level
            // ref : https://en.cppreference.com/w/cpp/language/if
            static_assert(dependent_false<T>::value, "Method \"at\" is not defined");
        }
    }

    size_t size() {
        auto call_size = METHOD(size);
        if constexpr (std::is_invocable_r_v<size_t, decltype(call_size), C<T>&>) {
            return C<T>::size();
        } else {
            static_assert(dependent_false<T>::value, "Method \"size\" is not defined");
        }
    }

    bool empty() {
        auto call_size = METHOD(empty);
        if constexpr (std::is_invocable_r_v<bool, decltype(call_size), C<T>&>) {
            return C<T>::empty();
        } else {
            static_assert(dependent_false<T>::value, "Method \"empty\" is not defined");
        }
    }

    void clear() {
        auto call_size = METHOD(clear);
        if constexpr (std::is_invocable_r_v<void, decltype(call_size), C<T>&>) {
            C<T>::clear();
        } else {
            static_assert(dependent_false<T>::value, "Method \"clear\" is not defined");
        }
    }

    void push_back(T t) {
        auto call_size = METHOD(push_back);
        if constexpr (std::is_invocable_r_v<void, decltype(call_size), C<T>&, T>) {
            C<T>::push_back(t);
        } else {
            static_assert(dependent_false<T>::value, "Method \"push_back\" is not defined");
        }
    }

    void push_front(T t) {
        auto call_size = METHOD(push_front);
        if constexpr (std::is_invocable_r_v<void, decltype(call_size), C<T>&, T>) {
            C<T>::push_front(t);
        } else {
            static_assert(dependent_false<T>::value, "Method \"push_front\" is not defined");
        }
    }

    void insert(typename C<T>::iterator pos, T t) {
        auto call_size = METHOD(insert);
        if constexpr (std::is_invocable_r_v<void, decltype(call_size), C<T>&, typename C<T>::iterator, T>) {
            C<T>::insert(pos, t);
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto begin() {
        auto call_size = METHOD(begin);
        if constexpr (std::is_invocable_r_v<typename C<T>::iterator, decltype(call_size), C<T>&>) {
            return C<T>::begin();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    auto end() {
        auto call_size = METHOD(end);
        if constexpr (std::is_invocable_r_v<typename C<T>::iterator, decltype(call_size), C<T>&>) {
            return C<T>::end();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    bool contains(T t) {
        if constexpr (std::is_same<C<T>, std::vector<T>>::value) {
            return std::find(std::vector<T>::begin(), std::vector<T>::end(), t) != std::vector<T>::end();
        } else if constexpr (std::is_same<C<T>, std::list<T>>::value) {
            return std::find(std::list<T>::begin(), std::list<T>::end(), t) != std::list<T>::end();
        } else {
            static_assert(dependent_false<T>::value, "Not a valid container");
        }
    }

    void print() {
        if constexpr (std::is_same<C<T>, std::vector<T>>::value || std::is_same<C<T>, std::list<T>>::value)  {
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

template<class T, template<typename...> class C, class ...Ps>
struct Container<T, C, Unique, Ps...> : public Container<T, C, Ps...> {
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
struct Container<T, C, Sorted, Ps...> : public Container<T, C, Ps...> {
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
    std::cout << "is empty? " << l1.empty() << std::endl;
    std::cout << typeid(l1).name() << std::endl;
    std::cout << "Container for default list" << std::endl;
    l1.print();
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

    return 0;
}