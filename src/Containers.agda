-- If we take the definition:
-- Containers are the infinitary generalization of normal functors.
module Containers where
  open import Data.Product as Prod using (_×_; _,_; Σ; uncurry)
  open import Data.Empty
  open import Data.Unit
  open import Function
  open import Vec
  -- A Container is given by a set of shapes and a family
  -- of positions assigning, to each shape, the set of positions where data can be stored in
  -- a data structure of that shape.
  record Container : Set₁ where
    constructor _◁_
    field
      Shape : Set
      Position : Shape → Set
  open Container public

  -- Extension :
  -- every container S ◁ P gives rise to a functor which maps a set A to the the set of pairs
  -- consisting of a choice of a shape s ∈ Shape and a function assigning to every position p ∈ Position s for that shape,
  -- an element of A to be stored at that position.
  ⟦_⟧ : Container → Set → Set
  ⟦ Sh ◁ Po ⟧ X = Σ Sh λ s → Po s → X

  -- Container morphism
  -- a container morphism is given by a pair of functions
  -- the first mapping input shapes to output shapes,
  -- and the second mapping output positions back to the input positions
  -- from which they fetch elements.
  -- for example reversing a list : list₁ → list₂, can be viewed as a container morphism
  _→ᶜ_ : Container → Container → Set
  (Sh₁ ◁ Po₁) →ᶜ (Sh₂ ◁ Po₂) = Σ (Sh₁ → Sh₂) λ f → (s : Sh₁) → Po₂ (f s) → Po₁ s

  _/ᶜ_ : ∀ {C₁ C₂} → C₁ →ᶜ C₂ → ∀ {X} → ⟦ C₁ ⟧ X → ⟦ C₂ ⟧ X
  (to , from) /ᶜ (Sh , f) = to Sh , f ∘ from Sh



  -- Closure Properties
  Kᶜ : Set → Container
  Kᶜ X = X ◁ λ _ → ⊥

  Iᶜ : Container
  Iᶜ = ⊤ ◁ λ _ → ⊤

  _+ᶜ_ : Container → Container → Container
  (Shape₁ ◁ Position₁) +ᶜ (Shape₂ ◁ Position₂) = (Shape₁ + Shape₂) ◁ uncurry (Position₁ ⟨?⟩ Position₂)

  _×ᶜ_ : Container → Container → Container
  (Shape₁ ◁ Position₁) ×ᶜ (Shape₂ ◁ Position₂) = (Shape₁ × Shape₂) ◁ uncurry λ x y →  Position₁ x + Position₂ y

  Σᶜ : (A : Set) → (C : A → Container) → Container
  Σᶜ A C = Σ A (λ a → Shape (C a)) ◁ uncurry λ x y → Position (C x) y

  Πᶜ : (A : Set) → (C : A → Container) → Container
  Πᶜ A C = ((a : A) → Shape (C a)) ◁ λ f → Σ A λ x → Position (C x) (f x)

  _oᶜ_ : Container → Container → Container
  (Sh ◁ Po) oᶜ C = Σᶜ Sh λ s → Πᶜ (Po s) λ p → C

  -- Containers yield endofunctor
  instance
    conEndoFunctor : {C : Container} → EndoFunctor ⟦ C ⟧
    conEndoFunctor {Sh ◁ Po} = record {map = λ f → uncurry λ s ps → s , f ∘ ps}
