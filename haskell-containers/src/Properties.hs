module Properties where
import ConLike

{- Properties -}
unique :: (Eq a, ConLike t) => t a -> Bool
unique c = for_all_unique_pairs c (/=)

ascending :: (Ord a, Pos t) => t a -> Bool
ascending c = for_all_unique_pairs c (ascendingComp c)

descending :: (Ord a, Pos t) => t a -> Bool
descending c = for_all_unique_pairs c (descendingComp c)

evenElm :: (Integral a, ConLike t) => t a -> Bool
evenElm c = for_all_elms c isEven

oddElm :: (Integral a, ConLike t) => t a -> Bool
oddElm c = for_all_elms c isOdd

-- potential usage
{-
type UniqueCon a = (Eq a, ConLike t) => {c :: t a | unique c}
-}

-- Observation : these two property functions do not have type Con a -> Bool
-- Question: can we use them as refinements on Con a?
lifo :: (ReadRemove t, Eq (t a)) => t a -> a -> Bool
lifo c x = remove (insertElm c x) == c

fifo :: (ReadRemove t, Eq (t a)) => t a -> a -> Bool
fifo c x = if isEmpty c
            then remove (insertElm c x) == c
            else remove (insertElm c x) == insertElm (remove c) x

{- Combinators-}
-- Unary Predicates
for_all_elms :: (ConLike t) => t a -> (a -> Bool) -> Bool
for_all_elms c f = reduce (&&) True (cmap (\a -> f a ) c)

-- Binary Predicates
for_all_unique_pairs :: (Eq a, ConLike t) => t a -> (a -> a -> Bool) -> Bool
for_all_unique_pairs c f = reduce (&&) True (flatten (cmap (\a -> (cmap (\b -> f a b) (removeElm c a))) c))

{- Helpers -}
cmap :: (ConLike t) => (a -> b) -> t a -> t b
cmap = undefined

reduce :: (ConLike t) => (b -> a -> b) -> b -> t a -> b
reduce = undefined

flatten :: (ConLike t) => t (t a) -> t a
flatten = undefined

ascendingComp :: (Ord a, Pos t) => t a -> a -> a -> Bool
ascendingComp c x y = if (position c x) < (position c y)
                        then x <= y
                        else x >= y

descendingComp :: (Ord a, Pos t) => t a -> a -> a -> Bool
descendingComp c x y = if (position c x) < (position c y)
                        then x >= y
                        else x <= y

isEven :: (Integral a) => a -> Bool
isEven x = x `mod` 2 == 0

isOdd :: (Integral a) => a -> Bool
isOdd x = x `mod` 2 == 1