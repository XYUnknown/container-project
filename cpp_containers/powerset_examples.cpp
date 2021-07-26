#include "powerset.hpp"

int main()
{
    Container<std::size_t, TreeSetWrapperAsc, Iterable> s1;
    s1.insert((std::size_t)2);
    s1.insert((std::size_t)3);
    s1.insert((std::size_t)5);
    s1.insert((std::size_t)7);
    auto pset1 = powerset<Container<std::size_t, TreeSetWrapperAsc, Iterable>, TreeSetWrapperAsc, Iterable>(s1);
 
    std::cout << "Using std::set:" << std::endl;
    for (auto&& subset: pset1) {
        std::cout << "{ ";
        char const* prefix = "";
        for (auto&& e: subset) {
            std::cout << prefix << e;
            prefix = ", ";
        }
        std::cout << " }\n";
    }

    Container<std::size_t, HashSetWrapper, Iterable> s2;
    s2.insert((std::size_t)2);
    s2.insert((std::size_t)3);
    s2.insert((std::size_t)5);
    s2.insert((std::size_t)7);
    auto pset2 = powerset<Container<std::size_t, HashSetWrapper, Iterable>, std::vector, Iterable>(s2);
 
    std::cout << "Using std::unordered_set:" << std::endl;
    for (auto&& subset: pset2) {
        std::cout << "{ ";
        char const* prefix = "";
        for (auto&& e: subset) {
            std::cout << prefix << e;
            prefix = ", ";
        }
        std::cout << " }\n";
    }

    Container<std::size_t, std::vector, Iterable> s3;
    s3.insert((std::size_t)2);
    s3.insert((std::size_t)3);
    s3.insert((std::size_t)5);
    s3.insert((std::size_t)7);
    auto pset3 = powerset<Container<std::size_t, std::vector, Iterable>, std::vector, Iterable>(s3);
 
    std::cout << "Using std::vector:" << std::endl;
    for (auto&& subset: pset3) {
        std::cout << "{ ";
        char const* prefix = "";
        for (auto&& e: subset) {
            std::cout << prefix << e;
            prefix = ", ";
        }
        std::cout << " }\n";
    }
}