module Lists.AbstractRefLists() where
import qualified Data.Set as S

{-@ type IncrList a = [a]<{\x y -> x <= y}> @-}
{-@ isIncList :: IncrList Int @-}
isIncList::[Int]
isIncList = [1, 2, 2, 3]

{-
{-@ notIncList :: IncrList Int @-}
notIncList::[Int]
notIncList = [3, 2, 2, 3]
-}

{-@ type UniqueList a = [a]<{\x y -> x != y}> @-}
{-@ isUniqueList :: UniqueList Int @-}
isUniqueList::[Int]
isUniqueList = [2, 1, 3]

{-
{-@ notUniqueList :: UniqueList Int @-}
notUniqueList::[Int]
notUniqueList = [2, 1, 2, 3]
-}

{-@ type StrictIncrList a = [a]<{\x y -> x != y && x <= y}> @-}
{-@ isStrictIncrList :: StrictIncrList Int @-}
isStrictIncrList::[Int]
isStrictIncrList = [1, 2, 3]

{-
{-@ notStrictIncrList :: StrictIncrList Int @-}
notStrictIncrList::[Int]
notStrictIncrList = [1, 2, 2, 3]
-}

{-@ pushI :: (Ord a) => IncrList a -> a -> IncrList a @-}
pushI [] y = y:[]
pushI (x:xs) y = case y <= x of 
    True -> y : (x : xs)
    _ -> x : pushI xs y

{-@ pushU :: (Ord a) => UniqueList a -> a -> UniqueList a @-}
pushU [] y = y:[]
pushU (x:xs) y = case y == x of
    True -> (x:xs)
    _ -> x: pushU xs y

{-@ pushS :: (Ord a) => StrictIncrList a -> a -> StrictIncrList a @-}
pushS [] y = y:[]
pushS (x:xs) y
    | y == x = (x:xs)
    | y < x = y : (x : xs)
    | y > x = x : pushS xs y