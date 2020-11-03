-- If we take the definition:
-- Containers are the infinitary generalization of normal functors.
module Containers where
  record Container : Set₁ where
    constructor _◁_
    field
      Shape : Set
      Position : Shape → Set

