module ConLike where
import Data.List

class ConLike t where
    empty :: t a
    isEmpty :: t a -> Bool
    insertElm :: t a -> a -> t a
    removeElm :: (Eq a) => t a -> a -> t a
    contains :: (Eq a) => t a -> a -> Bool
    size :: t a -> Int

class Pos t where
    position :: (Eq a) => t a -> a -> Maybe Int

instance ConLike [] where
    empty = [] -- c = empty::[a]
    isEmpty c = null c
    insertElm c x = x:c
    removeElm c x = delete x c
    contains c x = elem x c
    size c = length c

instance Pos [] where
    position c x = elemIndex x c