#include <set>
#include <iostream>
#include <numeric>
#include "containers_interface.hpp"
 
template <class S, template<class...> class C, class ...Ps>
Container<S, C, Ps...> powerset(S& s)
{
    Container<S, C, Ps...> ret;
    S empty;
    ret.insert(empty);
    for (auto&& e: s) {
        Container<S, C, Ps...> rs;
        for (auto x: ret) {
            x.insert(e);
            rs.insert(x);
        }
        for (auto x: rs) {
            ret.insert(x);
        }
    }
    return ret;
}

template<typename T>
struct hash_on_sum
: private std::hash<std::size_t>
{
  //typedef typename T::element_type count_type;
  typedef std::hash<std::size_t> base;
  std::size_t operator()(T const&obj) const
  {
    return base::operator()(std::accumulate(obj.begin(),obj.end(),std::size_t()));
  }
};

template<class T>
using HashSetWrapperH = WithProperty<T, std::unordered_set<T, hash_on_sum<T>>, Unique<>>;
