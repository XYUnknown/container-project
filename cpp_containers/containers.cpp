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

template <typename T, typename P>
class Vec /*: public Container<T, P>*/ {
    private:
    vector<T> v;

    public:
    Vec() /*: Container<T, P> ()*/ {}
    ~Vec() {}
    void push(T value);
    T pop();
    void clear();
    size_t size();
    bool is_empty();
    bool contains(T value);
};

template <typename T, typename P>
inline void Vec<T, P>::push(T value) {
    v.push_back(value);
}

template <typename T, typename P>
inline T Vec<T, P>::pop() {
    T value = v.back();
    v.pop_back();
    return value;
}

template <typename T, typename P>
inline void Vec<T, P>::clear() {
    v.clear();
}

template <typename T, typename P>
inline size_t Vec<T, P>::size() {
    return v.size();
}

template <typename T, typename P>
inline bool Vec<T, P>::is_empty() {
    return v.empty();
}

template <typename T, typename P>
inline bool Vec<T, P>::contains(T value) {
    return find(v.begin(), v.end(), value) != v.end();
}

template <typename T>
class Vec<T, Unique> {
    public:
    void push(T value);
};

template <typename T>
inline void Vec<T, Unique>::push(T value) {
    if (!this->contains(value)) {
        this->push(value);
    }
}

int main() {
    Vec<int, monostate> v1;
    v1.push(1);
    v1.push(1);

    Vec<int, Unique> v2;
    v2.push(1);
    v2.push(1);
    cout << v1.size() << endl;
    cout << v2.size() << endl; //1
    return 0;
}