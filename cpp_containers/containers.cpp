#include <iostream>
#include <variant>
#include <vector>
using namespace std;

class Unique {};
class Sorted {};

/*template <typename T, typename P>
class Container {
    public:
    virtual void push(T);
    virtual T pop();
    virtual void clear();
    virtual size_t size();
    virtual bool contains(T);
    virtual bool is_empty();

    Container(){}
    virtual ~Container() {}
};*/

template<typename T>
class Vec /*: public Container<T, P>*/ {
    private:
    vector<T> v;

    public:
    Vec() /*: Container<T, P> ()*/ {}
    ~Vec() {}
    template<typename P> void push(T value);
    T pop();
    void clear();
    size_t size();
    bool is_empty();
    bool contains(T value);
};

template<typename T>
template<typename P> void Vec<T>::push(T value) {
    v.push_back(value);
}

template<typename T>
T Vec<T>::pop() {
    T value = v.back();
    v.pop_back();
    return value;
}

template<typename T>
void Vec<T>::clear() {
    v.clear();
}

template<typename T>
size_t Vec<T>::size() {
    return v.size();
}

template<typename T>
bool Vec<T>::is_empty() {
    return v.empty();
}

template<typename T>
bool Vec<T>::contains(T value) {
    return find(v.begin(), v.end(), value) != v.end();
}

// This is valid
template<>
template<> void Vec<int>::push<Unique>(int value) {
    if (!this->contains(value)) {
        v.push_back(value);
    }
}

// This is invalid
/*
template<typename T>
template<> void Vec<T>::push<Unique>(T value) {
    if (!this->contains(value)) {
        v.push_back(value);
    }
}*/

int main() {
    Vec<int> v1;
    v1.push<monostate>(1);
    v1.push<monostate>(1);

    Vec<int> v2;
    v2.push<Unique>(1);
    v2.push<Unique>(1);

    cout << v1.size() << endl; //2
    cout << v2.size() << endl; //1
    return 0;
}