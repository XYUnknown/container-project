#include <algorithm>
#include <random>
#include <vector>

/** Data generation */
std::vector<std::size_t> Generate(std::size_t size) {
    std::minstd_rand generator;
    generator.seed(size);
    std::vector<std::size_t> v(size);
    std::generate(v.begin(), v.end(), generator);
    return v;
}

template<class T, template<typename...> class C, class ...Ps>
void Generate(Container<T, C, Ps...>& c, std::size_t size) {
    std::minstd_rand generator;
    generator.seed(size);
    for (std::size_t i=0; i < size; i++)
        c.insert((std::size_t)generator());
}