module Properties where
import Con

{- Properties -}
unique :: (Eq a) => Con a -> Bool
unique c = for_all_unique_pairs c (/=)

ascending :: (Ord a) => Con a -> Bool
ascending c = for_all_unique_pairs c (ascendingComp c)

descending :: (Ord a) => Con a -> Bool
descending c = for_all_unique_pairs c (descendingComp c)

evenElm :: (Integral a) => Con a -> Bool
evenElm c = for_all_elms c isEven

oddElm :: (Integral a) => Con a -> Bool
oddElm c = for_all_elms c isOdd

{- Combinators-}
-- Unary Predicates
for_all_elms :: Con a -> (a -> Bool) -> Bool
for_all_elms c f = reduce (&&) True (cmap (\a -> f a ) c)

-- Binary Predicates
for_all_unique_pairs :: (Eq a) => Con a -> (a -> a -> Bool) -> Bool
for_all_unique_pairs c f = reduce (&&) True (flatten (cmap (\a -> (cmap (\b -> f a b) (removeElm c a))) c))

{- Helpers -}
cmap :: (a -> b) -> Con a -> Con b
cmap = undefined

reduce :: (b -> a -> b) -> b -> Con a -> b
reduce = undefined

flatten :: Con (Con a) -> Con a
flatten = undefined

ascendingComp :: (Ord a) => Con a -> a -> a -> Bool
ascendingComp c x y = if (position c x) < (position c y)
                        then x <= y
                        else x >= y

descendingComp :: (Ord a) => Con a -> a -> a -> Bool
descendingComp c x y = if (position c x) < (position c y)
                        then x >= y
                        else x <= y

isEven :: (Integral a) => a -> Bool
isEven x = x `mod` 2 == 0

isOdd :: (Integral a) => a -> Bool
isOdd x = x `mod` 2 == 1 

-- Test run
c :: Con Int
c = insertElm (insertElm empty 1) 2