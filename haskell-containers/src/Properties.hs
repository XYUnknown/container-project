module Properties where
import Con

unique :: (Eq a) => Con a -> Bool
unique c = for_all_unique_pair c (/=)

for_all_unique_pair :: (Eq a) => Con a -> (a -> a -> Bool) -> Bool
for_all_unique_pair c f = reduce (&&) True (flatten (cmap (\a -> (cmap (\b -> f a b) (removeElm c a))) c))

cmap :: (a -> b) -> Con a -> Con b
cmap = undefined

reduce :: (b -> a -> b) -> b -> Con a -> b
reduce = undefined

flatten :: Con (Con a) -> Con a
flatten = undefined