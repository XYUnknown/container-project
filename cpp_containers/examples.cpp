// Usage testing of the containers implementations version 6
// should be replaced by some unit test
#include <string>
#include <string_view>
#include <cassert> // To be compiled on Ubuntu
#include <queue>
#include <deque>

#include "containers.hpp"
#include "rng.hpp"

int main() {
    Container<int, std::vector> v1;
    v1.insert(6);
    v1.push_back(3);
    v1.insert(v1.begin(), 5);
    v1.insert(v1.end(), 7);
    v1.pop_back();
    v1.erase(v1.begin());
    assert(v1.find(5) == v1.end());
    std::cout << typeid(v1).name() << std::endl;
    std::cout << "at position 0: " << v1.at(0) << std::endl;
    std::cout << "front: " << v1.front() << std::endl;
    std::cout << "back: " << v1.back() << std::endl;
    std::cout << "rbegin: " << *v1.rbegin() << std::endl;
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
    l1.pop_front();
    // std::cout << l1.at(0) << std::endl; // error
    std::cout << typeid(l1).name() << std::endl;
    std::cout << "Container for default list" << std::endl;
    l1.print();
    std::cout << "is empty? " << l1.empty() << std::endl;
    l1.clear();
    std::cout << "After clear" << std::endl;
    l1.print();

    Container<int, std::vector, Unique> v2;
    v2.reserve(10);
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

    Container<int, std::vector, Sorted<int>> v3;
    v3.insert(6);
    v3.insert(1);
    v3.insert(6);
    v3.insert(5);
    assert(v3.find(7) == v3.end());
    assert(!v3.contains(7));
    std::cout << "Container for sorted vector" << std::endl;
    v3.print();

    Container<int, std::vector, Sorted<int, std::greater<int>>> v3g;
    v3g.insert(6);
    v3g.insert(1);
    v3g.insert(6);
    v3g.insert(5);
    assert(v3g.find(7) == v3g.end());
    assert(!v3g.contains(7));
    std::cout << "Container for sorted vector (descending)" << std::endl;
    v3g.print();

    Container<int, std::list, Sorted<int>> l3;
    l3.insert(6);
    l3.insert(4);
    l3.insert(1);
    l3.insert(4);
    assert(l3.find(7) == l3.end());
    assert(!l3.contains(7));
    std::cout << "Container for sorted list" << std::endl;
    l3.print();

    Container<int, std::list, Sorted<int, std::greater<int>>> l3g;
    l3g.insert(6);
    l3g.insert(4);
    l3g.insert(1);
    l3g.insert(4);
    assert(l3g.find(7) == l3g.end());
    assert(!l3g.contains(7));
    std::cout << "Container for sorted list (descending)" << std::endl;
    l3g.print();

    Container<int, std::vector, Sorted<int>, Unique> v4;
    v4.insert(6);
    v4.insert(1);
    v4.insert(1);
    std::cout << "Container for sorted unique vector" << std::endl;
    v4.print();

    Container<int, std::vector, Sorted<int, std::greater<int>>, Unique> v4g;
    v4g.insert(6);
    v4g.insert(1);
    v4g.insert(1);
    std::cout << "Container for sorted (descending) unique vector" << std::endl;
    v4g.print();

    Container<int, std::list, Sorted<int>, Unique> l4;
    l4.insert(6);
    l4.insert(1);
    l4.insert(1);
    std::cout << "Container for sorted unique list" << std::endl;
    l4.print();

    Container<int, std::list, Sorted<int, std::greater<int>>, Unique> l4g;
    l4g.insert(6);
    l4g.insert(1);
    l4g.insert(1);
    std::cout << "Container for sorted unique list" << std::endl;
    l4g.print();

    Container<int, std::vector, Unique, Sorted<int>> v5;
    v5.insert(6);
    v5.insert(1);
    v5.insert(1);
    std::cout << "Container for unique sorted vector" << std::endl;
    v5.print();

    Container<int, std::list, Unique, Sorted<int>> l5;
    l5.insert(6);
    l5.insert(1);
    l5.insert(1);
    std::cout << "Container for unique sorted list" << std::endl;
    l5.print();

    Container<int, std::vector, SortedOnAccess<int>> v6;
    v6.insert(6);
    v6.insert(1);
    v6.insert(6);
    v6.insert(5);
    assert(v6.find(7) == v6.end());
    assert(!v6.contains(7));
    std::cout << "Container for sorted (on access) vector" << std::endl;
    v6.print();

    Container<int, std::vector, SortedOnAccess<int, std::greater<int>>> v7;
    v7.insert(6);
    v7.insert(1);
    v7.insert(6);
    v7.insert(5);
    assert(v7.find(7) == v7.end());
    assert(!v7.contains(7));
    std::cout << "Container for sorted (on access) vector" << std::endl;
    v7.print();

    Container<int, std::list, SortedOnAccess<int>> l6;
    l6.insert(6);
    l6.insert(4);
    l6.insert(1);
    l6.insert(4);
    assert(l6.find(7) == l6.end());
    assert(!l6.contains(7));
    std::cout << "Container for sorted (on access) list" << std::endl;
    l6.print();

    Container<int, std::list, SortedOnAccess<int, std::greater<int>>> l7;
    l7.insert(6);
    l7.insert(4);
    l7.insert(1);
    l7.insert(4);
    assert(l7.find(7) == l7.end());
    assert(!l7.contains(7));
    std::cout << "Container for sorted (on access) list" << std::endl;
    l7.print();

    TreeSetWrapper<int> sw;
    sw.size();
    static_assert(sw.has_property<Unique>()); // not recommended to call in this way

    Container<int, TreeSetWrapper> s1;
    s1.insert(6);
    s1.insert(1);
    s1.insert(1);
    assert(s1.find(1) != s1.end());
    std::cout << "Container for set(tree set) is sorted and unique" << std::endl;
    s1.print();

    Container<int, TreeSetWrapper, Unique> s2;
    s2.insert(6);
    s2.insert(1);
    s2.insert(1);
    assert(s2.find(1) != s2.end());
    std::cout << "Container for unique set(tree set) is sorted and unique" << std::endl;
    s2.print();

    Container<int, TreeSetWrapper, Sorted<int>> s3;
    s3.insert(6);
    s3.insert(1);
    s3.insert(1);
    assert(s1.find(1) != s1.end());
    std::cout << "Container for sorted set(tree set) is sorted and unique" << std::endl;
    s3.print();

    Container<int, TreeSetWrapper, Unique, Sorted<int>> s4;
    s4.insert(6);
    s4.insert(1);
    s4.insert(1);
    assert(s2.find(1) != s2.end());
    std::cout << "Container for unique and sorted set(tree set) is sorted and unique" << std::endl;
    s4.print();

    TreeWrapper<int> tw;
    tw.size();
    static_assert(tw.has_property<Sorted<int>>());
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

    Container<int, TreeWrapper, Sorted<int>> t3;
    t3.insert(6);
    t3.insert(1);
    t3.insert(1);
    std::cout << "Container for sorted tree (multiset) is sorted" << std::endl;
    t3.print();

    Container<int, TreeWrapper, Unique, Sorted<int>> t4;
    t4.insert(6);
    t4.insert(1);
    t4.insert(1);
    std::cout << "Container for unique sorted tree (multiset) is unique and sorted" << std::endl;
    t4.print();
    
    Container<int, HashSetWrapper> hs1;
    hs1.insert(6);
    hs1.insert(1);
    hs1.insert(1);
    hs1.insert(7);
    hs1.insert(9);
    hs1.insert(7);
    assert(hs1.contains(1));
    std::cout << "Container for hash set (unordered_set) is unique" << std::endl;
    hs1.print();

    Container<int, HashSetWrapper, Unique> hs2;
    hs2.insert(6);
    hs2.insert(1);
    hs2.insert(1);
    hs2.insert(7);
    hs2.insert(9);
    hs2.insert(7);
    assert(hs2.contains(1));
    std::cout << "Container for unique hash set (unordered_set) is unique" << std::endl;
    hs2.print();

    /* This doesn't make sense
    Container<int, HashSetWrapper, Sorted> hs3;
    hs3.insert(6);
    hs3.insert(1);
    hs3.insert(1);
    hs3.insert(7);
    hs3.insert(9);
    hs3.insert(7);
    std::cout << "Container for sorted hash set (unordered_set) is unique and sorted" << std::endl;
    hs3.print();*/

    // Nested containers
    Container<Container<int, std::vector, Unique>, std::vector, Sorted<Container<int, std::vector, Unique>>> n1;
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
    std::cout << "Size: " << "Nested container, a sorted vector of unique vectors" << std::endl;
    std::cout << "Size: " << n1.size() << std::endl;
    for (auto it=n1.begin(); it!=n1.end(); it++)
        it->print();

    Container<Container<int, std::vector, Sorted<int>>, std::vector, Unique> n2;
    Container<int, std::vector, Sorted<int>> e4, e5, e6;
    e4.insert(4);
    e4.insert(3);
    e5.insert(5);
    e5.insert(1);
    e5.insert(10);
    e6.insert(11);
    n2.insert(e4);
    n2.insert(e4);
    n2.insert(e5);
    n2.insert(e6);
    std::cout << "Size: " << "Nested container, a unique vector of sorted vectors" << std::endl;
    std::cout << "Size: " << n2.size() << std::endl;
    for (auto it=n2.begin(); it!=n2.end(); it++)
        it->print();
    
    // test is present
    static_assert(!is_present<Unique, Sorted<int>>());
    static_assert(is_present<Unique, Unique, Sorted<int>>());
    static_assert(is_present<Sorted<int>, Unique, Sorted<int>>());

    // test the make_container
    auto c1 = make_container<int, Unique>();
    c1.insert(6);
    c1.insert(1);
    c1.insert(1);
    c1.insert(7);
    c1.insert(7);
    std::cout << "Unique Container from function make_container" << std::endl;
    c1.print();

    auto c2 = make_container<int, Sorted<int>>();
    c2.insert(6);
    c2.insert(1);
    c2.insert(1);
    c2.insert(7);
    c2.insert(7);
    std::cout << "Sorted Container from function make_container" << std::endl;
    c2.print();

    auto c3 = make_container<int, Unique, Sorted<int>>();
    c3.insert(6);
    c3.insert(1);
    c3.insert(1);
    c3.insert(7);
    c3.insert(7);
    std::cout << "Unique Sorted Container from function make_container" << std::endl;
    c3.print();

    // maps
    /*Container<std::pair<std::string, int>, TreeMapWrapper> m1;
    m1.insert({"a", 1});
    m1.insert({"b", 2});
    m1.insert_or_assign("b", 9);
    
    std::cout << "Container for a tree map" << std::endl;
    std::cout << "has key \"a\"? " << m1.contains("a") << std::endl;
    m1.print();*/

    // a vector of pairs
    Container<std::pair<std::string, int>, std::vector, Sorted<std::pair<std::string, int>>> vp1;
    vp1.insert({"c", 1});
    vp1.insert({"b", 2});
    vp1.print();

    Container<std::pair<std::string, int>, TreeWrapper, Sorted<std::pair<std::string, int>>> tp1;
    tp1.insert({"c", 1});
    tp1.insert({"b", 2});
    tp1.print();

    // rng for std::pair<std::size_t, std::string>
    std::pair<std::size_t, std::string> p = generate_pair(5, 10000);
    std::cout << std::get<0>(p) << ", " << std::get<1>(p) << std::endl;
    std::cout << sizeof(p) << std::endl;

    std::vector<std::pair<std::size_t, std::string>> vpp = generate_pairs(5, 10);
    for (auto it=vpp.begin(); it!=vpp.end(); it++)
        std::cout << " <"<< std::get<0>(*it) << ", " << std::get<1>(*it)<< ">";
    std::cout << vpp.size() << std::endl;

    std::priority_queue<std::pair<std::size_t, std::string>, std::vector<std::pair<std::size_t, std::string>>> q1(vpp.begin(), vpp.end());
    std::priority_queue<std::pair<std::size_t, std::string>, std::deque<std::pair<std::size_t, std::string>>> q2(vpp.begin(), vpp.end());
    Container<std::pair<std::size_t, std::string>, std::vector, SortedOnAccess<std::pair<std::size_t, std::string>>> q3;
    for (std::pair<std::size_t, std::string> e: vpp)
        q3.insert(e);
    std::cout << " <"<< std::get<0>(q1.top()) << ", " << std::get<1>(q1.top())<< ">" << std::endl;
    std::cout << " <"<< std::get<0>(q2.top()) << ", " << std::get<1>(q2.top())<< ">" << std::endl;
    std::cout << " <"<< std::get<0>(q3.back()) << ", " << std::get<1>(q3.back())<< ">" << std::endl;

    return 0;
}
