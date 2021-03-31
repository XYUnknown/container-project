{-# LANGUAGE NoMonomorphismRestriction #-}
{-@ LIQUID "--notermination" @-}
{-@ LIQUID "--exact-data-cons" @-}

module Demo.UniqueList () where
import qualified Data.Set as S
import Prelude hiding (length, map, filter, head, tail, foldl1)
import Language.Haskell.Liquid.Prelude (liquidError)

{-@ measure elts @-}
elts :: (Ord a) => [a] -> S.Set a
elts [] = S.empty
elts (x:xs) = S.singleton x `S.union` elts xs

{-@ predicate EqElts X Y = ((elts X) = (elts Y)) @-}
{-@ predicate Disjoint X Y = (Set_emp (S.union (elts X) (elts Y))) @-}
{-@ predicate Union X Y Z = ((elts X) = (S.union (elts Y) (elts Z))) @-}
{-@ predicate UnionElem X Y Z = ((elts X) = (S.union (elts Y) (S.singleton Z))) @-}
    
{-@ measure unique @-}
unique :: (Ord a) => [a] -> Bool
unique [] = True
unique (x:xs) = unique xs && not (S.member x (elts xs))

{-@ type UniqueList a = {v:[a] | unique v }@-}

{-@ isUniqueListEmpty :: UniqueList Int @-}
isUniqueListEmpty::[Int]
isUniqueListEmpty = []

{-@ isUnique :: UniqueList Int @-}
isUnique::[Int]
isUnique = [1, 2, 3] 

--{-@ isNotUnique :: UniqueList Int @-}
--isNotUnique::[Int]
--isNotUnique = [1, 2, 3, 1]

{-@ append :: xs: (UniqueList a) 
              -> ys: {v: (UniqueList a) | Disjoint v xs} 
              -> {v: UniqueList a | (Union v xs ys)} @-}
append [] ys = ys
append (x:xs) ys = x: append xs ys

{-@ push :: xs: (UniqueList a) 
            -> y: {v:a | not (S.member v (elts xs))}
            -> {v: UniqueList a | (UnionElem v xs y)} @-}
push [] y = [y]
push (x:xs) y = x: push xs y
