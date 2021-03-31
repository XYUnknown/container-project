{-# LANGUAGE NoMonomorphismRestriction #-}
{-@ LIQUID "--notermination" @-}
{-@ LIQUID "--exact-data-cons" @-}

module Demo.IncrList () where
import qualified Data.Set as S

{-@ measure ascending @-}
ascending :: (Ord a) => [a] -> Bool
ascending [] = True
ascending (x:xs) = case xs of
    [] -> True
    (y:_) -> x<=y && ascending xs

-- {-@ type IncrList a = [a]<{\x y -> x <= y}> @-} 
{-@ type IncrList a = {v:[a] | ascending v} @-} 
{-@ isIncListEmpty :: IncrList Int @-}
isIncListEmpty::[Int]
isIncListEmpty = []

{-@ isIncList :: IncrList Int @-}
isIncList::[Int]
isIncList = [1, 2, 2, 3]

--{-@ notIncList :: IncList Int @-}
--notIncList::[Int]
--notIncList = [1, 2, 2, 3, 1]

{-@ insert :: (Ord a) => a -> IncrList a -> IncrList a @-}
insert y [] = y:[]
insert y (x:xs)
    | y <= x = y : (x : xs)
    | otherwise = x : insert y xs

{-
{-@ pushI :: (Ord a) => a -> IncrList a -> IncrList a @-}
pushI y []     = [y]
pushI y (x:xs)
    | y <= x      = y : x : xs
    | otherwise   = x : pushI y xs
-}