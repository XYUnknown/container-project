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

std::pair<std::size_t, std::string> generate_pair(std::size_t len, std::size_t seed) {
    static constexpr auto chars =
        "0123456789"
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        "abcdefghijklmnopqrstuvwxyz";
    std::minstd_rand generator;
    generator.seed(seed);
    auto dist = std::uniform_int_distribution{{}, std::strlen(chars) - 1};
    auto result = std::string(len, '\0');
    std::generate_n(begin(result), len, [&]() { return chars[dist(generator)]; });
    return std::pair<std::size_t, std::string>((std::size_t)generator(), result);
}

std::vector<std::pair<std::size_t, std::string>> generate_pairs(std::size_t len, std::size_t size) {
    std::vector<std::pair<std::size_t, std::string>> v;
    static constexpr auto chars =
        "0123456789"
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        "abcdefghijklmnopqrstuvwxyz";
    std::minstd_rand generator;
    generator.seed(size);
    for (std::size_t i=0; i < size; i++) {
        auto dist = std::uniform_int_distribution{{}, std::strlen(chars) - 1};
        auto result = std::string(len, '\0');
        std::generate_n(begin(result), len, [&]() { return chars[dist(generator)]; });
        v.push_back(std::pair<std::size_t, std::string>((std::size_t)generator(), result));
    }
    return v;
}