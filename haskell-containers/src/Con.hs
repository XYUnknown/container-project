module Con (Con, empty, isEmpty, insertElm, removeElm, contains, size, position) where
import Data.List

empty :: Con a
isEmpty :: Con a -> Bool
insertElm :: Con a -> a -> Con a
removeElm :: (Eq a) => Con a -> a -> Con a
contains :: (Eq a) => Con a -> a -> Bool
size :: Con a -> Int

newtype Con a = ConImpl [a] deriving (Eq, Show)
empty = ConImpl []
isEmpty (ConImpl c) = null c
insertElm (ConImpl c) x = ConImpl (x:c)
removeElm (ConImpl c) x = ConImpl (delete x c)
contains (ConImpl c) x = elem x c
size (ConImpl c) = length c

class Pos t where
    position :: (Eq a) => t a -> a -> Maybe Int

instance Pos Con where
    position (ConImpl c) x = elemIndex x c