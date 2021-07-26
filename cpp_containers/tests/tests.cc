#include <gtest/gtest.h>
#include <vector>
#include <list>
#include <set>
#include <string>
#include <string_view>

#include "containers_interface.hpp"
//#include "containers.hpp"

TEST(ContainerTest, VecInsertAssertion) {
    Container<int, std::vector> c;
    c.insert(1);
    EXPECT_EQ(c.empty(), false);
}

TEST(ContainerTest, SetInsertAssertion) {
    Container<int, TreeSetWrapperAsc> c;
    c.insert(1);
    EXPECT_EQ(c.empty(), false);
}

TEST(ContainerTest, VecPeekAssertion) {
    Container<int, std::vector> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 1);
}

TEST(ContainerTest, SetAscPeekAssertion) {
    Container<int, TreeSetWrapperAsc> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 3);
}

TEST(ContainerTest, SetDescPeekAssertion) {
    Container<int, TreeSetWrapperDesc> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 1);
}

TEST(ContainerTest, VecPopAssertion) {
    Container<int, std::vector> c;
    c.insert(3);
    c.insert(1);
    c.pop();
    EXPECT_EQ(c.size(), 1);
}

TEST(ContainerTest, SetPopAssertion) {
    Container<int, TreeSetWrapperAsc> c;
    c.insert(3);
    c.insert(1);
    c.pop();
    EXPECT_EQ(c.size(), 1);
}

TEST(ContainerTest, VecClearAssertion) {
    Container<int, std::vector> c;
    c.insert(3);
    c.insert(1);
    c.clear();
    EXPECT_EQ(c.empty(), true);
}

TEST(ContainerTest, SetClearAssertion) {
    Container<int, TreeSetWrapperAsc> c;
    c.insert(3);
    c.insert(1);
    c.clear();
    EXPECT_EQ(c.empty(), true);
}

TEST(LookUpContainerTest, VectorAtAssertion) {
    Container<int, std::vector, LookUp> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.at(0), 3);
}

TEST(LookUpContainerTest, SetAtAssertion) {
    Container<int, TreeSetWrapperAsc, LookUp, Iterable> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.at(0), 1);
}

TEST(IterableContainerTest, VectorInsertPosAssertion) {
    Container<int, std::vector, LookUp, Iterable> c;
    c.insert(3);
    c.insert(c.begin(), 1);
    EXPECT_EQ(c.at(0), 1);
}

TEST(IterableContainerTest, VectorContainsAssertion) {
    Container<int, std::vector, LookUp, Iterable> c;
    c.insert(3);
    c.insert(c.begin(), 1);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.contains(5), false);
}

TEST(IterableContainerTest, SetContainsAssertion) {
    Container<int, TreeSetWrapperAsc, LookUp, Iterable> c;
    c.insert(3);
    c.insert(c.begin(), 1);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.contains(5), false);
}

TEST(IterableContainerTest, VectorFindAssertion) {
    Container<int, std::vector, LookUp, Iterable> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.find(3), c.begin());
}

TEST(IterableContainerTest, SetFindAssertion) {
    Container<int, TreeSetWrapperAsc, LookUp, Iterable> c;
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.find(1), c.begin());
}

TEST(UniqueContainerTest, VectorInsertAssert) {
    Container<int, std::vector, Unique, LookUp, Iterable> c;
    c.insert(3);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.size(), 2);
}

TEST(UniqueContainerTest, SetInsertAssert) {
    Container<int, TreeSetWrapperAsc, Unique, LookUp, Iterable> c;
    c.insert(3);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.size(), 2);
}

TEST(ContainerAdapterTests, StackAssert) {
    Container<int, std::vector, Sorted<int, LIFO>> c;
    c.insert(3);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 1);
    c.pop();
    EXPECT_EQ(c.peek(), 3);
    c.clear();
    EXPECT_EQ(c.empty(), true);
}

TEST(ContainerAdapterTests, QueueAssert) {
    Container<int, std::list, Sorted<int, FIFO>> c;
    c.insert(2);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 2);
    c.pop();
    EXPECT_EQ(c.peek(), 3);
    c.clear();
    EXPECT_EQ(c.empty(), true);
}

TEST(ContainerAdapterTests, PriorityQueueAssert) {
    Container<int, std::vector, Sorted<int, HeapOrd<int, std::greater<int>>>>c;
    c.insert(2);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.peek(), 1);
    c.pop();
    EXPECT_EQ(c.peek(), 2);
    c.clear();
    EXPECT_EQ(c.empty(), true);
}

TEST(SortedContainerTests, SetAssert) {
    Container<int, TreeSetWrapperAsc, Sorted<int, std::greater<int>>, LookUp, Iterable>c;
    c.insert(2);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(*c.begin(), 1);
    EXPECT_EQ(c.peek(), 3);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.find(4), c.end());
}

TEST(SortedContainerTests, VectorAssert) {
    Container<int, std::vector, Sorted<int, std::greater<int>>, LookUp, Iterable>c;
    c.insert(2);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(*c.begin(), 3);
    EXPECT_EQ(c.peek(), 1);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.find(4), c.end());
}

TEST(SortedUniqueContainerTests, VectorAssert) {
    Container<int, std::vector, Sorted<int, std::greater<int>>, Unique, LookUp, Iterable>c;
    c.insert(2);
    c.insert(3);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(c.size(), 3);
    EXPECT_EQ(*c.begin(), 3);
    EXPECT_EQ(c.peek(), 1);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.find(4), c.end());
}

TEST(SortedLazyContainerTests, VectorAssert) {
    Container<int, std::vector, Sorted<int, std::greater<int>, false>, LookUp, Iterable>c;
    c.insert(2);
    c.insert(3);
    c.insert(1);
    EXPECT_EQ(*c.begin(), 3);
    EXPECT_EQ(c.peek(), 1);
    EXPECT_EQ(c.contains(3), true);
    EXPECT_EQ(c.find(4), c.end());
}

TEST(MapContainerTests, OrderedMap) {
    Container<std::pair<int, std::string>, TreeMapWrapperAsc, Map>c;
    c.insert({0, "hello"});
    EXPECT_EQ(c.empty(), false);
    EXPECT_EQ(c.size(), 1);
    EXPECT_EQ(c.lookup(1), std::nullopt);
    EXPECT_EQ(c.lookup(0), "hello");
    EXPECT_EQ(c.contains(1), false);
    EXPECT_EQ(c.at(0), "hello");
}

TEST(MapContainerTests, IterableOrderedMap) {
    Container<std::pair<int, std::string>, TreeMapWrapperAsc, Iterable, Map>c;
    c.insert({0, "hello"});
    EXPECT_EQ(c.empty(), false);
    EXPECT_EQ(c.size(), 1);
    EXPECT_EQ(c.lookup(1), std::nullopt);
    EXPECT_EQ(c.lookup(0), "hello");
    EXPECT_EQ(c.contains(1), false);
    EXPECT_EQ(c.at(0), "hello");
    c.insert({10, "hello"});
    EXPECT_EQ(c.begin()->first, 0);
    EXPECT_EQ(c.begin()->second, "hello");
}

TEST(UnorderedContainerTests, IterableUnOrderedSet) {
    Container<int, HashSetWrapper, Iterable>c;
    c.insert(0);
    EXPECT_EQ(c.empty(), false);
}