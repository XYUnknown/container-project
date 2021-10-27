{-# LANGUAGE ConstrainedClassMethods #-}
module ConLike where
import Data.List

-- Basic Container and Operations
class ConLike t where
    empty :: t a
    isEmpty :: t a -> Bool
    insertElm :: t a -> a -> t a
    removeElm :: (Eq a) => t a -> a -> t a
    contains :: (Eq a) => t a -> a -> Bool
    size :: t a -> Int

class (ConLike t) => Pos t where
    position :: (Eq a) => t a -> a -> Maybe Int

class (ConLike t) => ReadRemove t where
    read :: t a -> a
    remove :: t a -> t a

-- Prototype
-- Can we model the specification on library programmer side in this form?
class (ReadRemove t) => Stack t where
    axiom :: (Eq (t a)) => t a -> a -> Bool
    axiom c x = remove (insertElm c x) == c

class (ConLike t) => UniqueCon t where
    -- Question : How to check complete?
    axiomEq :: (Eq (t a)) => t a -> a -> Bool
    axiomEq c x = insertElm (insertElm c x) x == insertElm c x

    axiomNeq :: (Eq (t a), Eq a) => t a -> a -> a -> Bool
    axiomNeq c x y = if x /= y
                        then size (insertElm (insertElm c x) y) == size (insertElm c x) + 1
                        else size (insertElm (insertElm c x) y) == size (insertElm c x)

-- example implementation of a container using a list
instance ConLike [] where
    empty = [] -- c = empty::[a]
    isEmpty c = null c
    insertElm c x = x:c
    removeElm c x = delete x c
    contains c x = elem x c
    size c = length c

instance Pos [] where
    position c x = elemIndex x c