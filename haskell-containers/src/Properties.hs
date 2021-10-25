module Properties where
import Con

unique :: (Eq a) => Con a -> Bool
unique c = for_all_unique_pairs c (/=)

ascending :: (Ord a) => Con a -> Bool
ascending c = for_all_unique_pairs c (ascendingComp c)

descending :: (Ord a) => Con a -> Bool
descending c = for_all_unique_pairs c (descendingComp c)

for_all_unique_pairs :: (Eq a) => Con a -> (a -> a -> Bool) -> Bool
for_all_unique_pairs c f = reduce (&&) True (flatten (cmap (\a -> (cmap (\b -> f a b) (removeElm c a))) c))

cmap :: (a -> b) -> Con a -> Con b
cmap = undefined

reduce :: (b -> a -> b) -> b -> Con a -> b
reduce = undefined

flatten :: Con (Con a) -> Con a
flatten = undefined

-- TODO: implement it need as a operations introduced by a syntactic property
position :: Con a -> a -> Int -- the position is an integer
position = undefined

ascendingComp :: (Ord a) => Con a -> a -> a -> Bool
ascendingComp c x y = if (position c x) < (position c y)
                        then x <= y
                        else x >= y

descendingComp :: (Ord a) => Con a -> a -> a -> Bool
descendingComp c x y = if (position c x) < (position c y)
                        then x >= y
                        else x <= y