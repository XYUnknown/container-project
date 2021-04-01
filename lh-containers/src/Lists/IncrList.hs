{-# LANGUAGE NoMonomorphismRestriction #-}
{-@ LIQUID "--notermination" @-}
{-@ LIQUID "--exact-data-cons" @-}

module Lists.IncrList () where
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

{- not working
{-@ push :: (Ord a) => IncrList a -> a -> IncrList a @-}
push [] y = y:[]
push (x:xs) y = case y <= x of 
    True -> y : (x : xs)
    _ -> x : push xs y
-}
