module Vec where
  open import Data.Product as Prod using (_×_; _,_; Σ; uncurry)
  open import Function
  open import Data.Unit
  open import Level hiding (zero; suc)
  open import Relation.Binary.PropositionalEquality as Eq using (_≡_; subst; refl)

  data ℕ : Set where
    zero : ℕ
    suc : ℕ → ℕ
  {- BUILTIN NATURAL Nat -}

  _+ℕ_ : ℕ → ℕ → ℕ
  zero +ℕ y = y
  suc x +ℕ y = suc (x +ℕ y)

  _×ℕ_ : ℕ → ℕ → ℕ
  zero ×ℕ y = zero
  suc x ×ℕ y = y +ℕ (x ×ℕ y)

  -- definition of a list
  data List (X : Set) : Set where
    [] : List X
    _∷_ : X → List X → List X
  infixr 4 _∷_

  length : {X : Set} → List X → ℕ
  length [] = zero
  length (x ∷ xs) = suc (length xs)

  data Vec (X : Set) : ℕ → Set where
    [] : Vec X zero
    _∷_ : {n : ℕ} → X → Vec X n → Vec X (suc n)

  zip : ∀ {n S T} → Vec S n → Vec T n → Vec (S × T) n
  zip [] [] = []
  zip (x ∷ xs) (y ∷ ys) = (x , y) ∷ (zip xs ys)

  vec : forall {n X} → X → Vec X n
  vec {zero} x = []
  vec {suc n} x = x ∷ vec x

  vapp : forall {n S T} → Vec (S → T) n → Vec S n → Vec T n
  vapp fs [] = []
  vapp (f ∷ fs) (x ∷ ss) = f x ∷ vapp fs ss

  vmap : forall {n S T} → (S → T) → Vec S n → Vec T n
  vmap f ss = vapp (vec f) ss

  zip′ : ∀ {n S T} → Vec S n → Vec T n → Vec (S × T) n
  zip′ ss ts = vapp (vapp (vec (_,_)) ss) ts

  _++_ : ∀ {m n X} → Vec X m → Vec X n → Vec X (m +ℕ n)
  [] ++ ys = ys
  (x ∷ xs) ++ ys = x ∷ (xs ++ ys)

  takeVec : ∀ {m n X} → Vec X (m +ℕ n) → Vec X m
  takeVec {zero} xs = []
  takeVec {suc m} (x ∷ xs) = x ∷ takeVec xs

  dropVec : ∀ {m n X} → Vec X (m +ℕ n) → Vec X n
  dropVec {zero} xs = xs
  dropVec {suc m} (x ∷ xs) = dropVec xs

  splitVec : ∀ m n {X} → Vec X (m ×ℕ n) → Vec (Vec X n) m
  splitVec zero n xs = []
  splitVec (suc m) n xs = takeVec xs ∷ splitVec m n (dropVec xs)

  joinVec : ∀ m n {X} → Vec (Vec X n) m → Vec X (m ×ℕ n)
  joinVec zero n [] = []
  joinVec (suc m) n (xs ∷ xss) = xs ++ joinVec m n xss

  data Fin : ℕ → Set where
    zero : {n : ℕ} → Fin (suc n)
    suc : {n : ℕ} → Fin n → Fin (suc n)

  proj : ∀ {n X} → Vec X n → Fin n → X
  proj (x ∷ xs) zero = x
  proj (x ∷ xs) (suc i) = proj xs i

  tabulate : ∀ {n X} → (Fin n → X) → Vec X n
  tabulate {zero} f = []
  tabulate {suc n} f = f zero ∷ tabulate (f ∘ suc)

  record EndoFunctor (F : Set → Set) : Set₁ where
    field
      map : ∀ {S T} → (S → T) → F S → F T
  open EndoFunctor ⦃...⦄ public

  record Applicative (F : Set → Set) : Set₁ where
    infixl 2 _⊛_
    field
      pure : ∀ {X} → X → F X
      _⊛_ : ∀ {S T} → F (S → T) → F S → F T
    applicativeEndoFunctor : EndoFunctor F
    applicativeEndoFunctor = record {map = _⊛_ ∘ pure}
  open Applicative ⦃...⦄ public

  -- in Agda 2.6.x instance need to be specified to allow the
  -- usage of applicativeEndoFunctor in the definition of endoFunctorVec
  instance
    applicativeVec : ∀ {n} → Applicative λ X → Vec X n
    applicativeVec = record {pure = vec; _⊛_ = vapp}

    applicativeFun : ∀ {S} → Applicative λ X → S → X
    applicativeFun = record
      {pure = λ x s → x; -- also known as K, drop environment
       _⊛_ = λ f a s → f s (a s) -- also known as S, share environment
      }

    endoFunctorVec : ∀ {n} → EndoFunctor λ X → Vec X n
    endoFunctorVec = applicativeEndoFunctor

  record Monad (F : Set → Set) : Set₁ where
    field
      return : ∀ {X} → X → F X
      _>>=_ : ∀ {S T} → F S → (S → F T) → F T
    monadApplicative = record
      {pure = return;
       _⊛_ = λ ff fs → ff >>= λ f → fs >>= λ s → return (f s)
      }
  open Monad ⦃...⦄ public

  instance
    monadVec : {n : ℕ} → Monad λ X → Vec X n
    monadVec = record
      {return = vec;
       _>>=_ = λ xs f → diag (vmap f xs)
      } where
        tail : ∀ {n X} → Vec X (suc n) → Vec X n
        tail (x ∷ xs) = xs
        diag : ∀ {n X} → Vec (Vec X n) n → Vec X n
        diag [] = []
        diag ((x ∷ xs) ∷ xss) = x ∷ diag (vmap tail xss)

    applicativeId : Applicative id
    applicativeId = record
      {pure = id;
       _⊛_ = id
      }
    endoFunctor : EndoFunctor id
    endoFunctor = applicativeEndoFunctor

  applicativeComp : ∀ {F G} → Applicative F → Applicative G → Applicative (F ∘ G)
  applicativeComp aF aG = record
    {pure = λ x → (pure ⦃ aF ⦄ ∘ pure ⦃ aG ⦄) (pure x);
     _⊛_ = λ f s →  (pure ⦃ aF ⦄ (_⊛_ ⦃ aG ⦄) ** f) ** s
    } where
      _**_ = _⊛_ ⦃ aF ⦄

  record Monoid (X : Set) : Set where
    infixr 4 _∙_
    field
      ε : X
      _∙_ : X → X → X
    monoidApplicative : Applicative λ _ → X
    monoidApplicative = record
      {pure = λ x → ε;
       _⊛_ = _∙_
      }
  open Monoid ⦃...⦄ public

  record Traversable (F : Set → Set) : Set₁ where
    field
      traverse : ∀ {G S T} ⦃ AG : Applicative G ⦄ → (S → G T) → F S → G (F T)
    traverseEndoFunctor : EndoFunctor F
    traverseEndoFunctor = record {map = traverse}
  open Traversable ⦃...⦄ public

  instance
    traversableVec : ∀ {n} → Traversable λ X → Vec X n
    traversableVec = record {traverse = vtr} where
      vtr : ∀ {n G S T} ⦃ _ : Applicative G ⦄ → (S → G T) → Vec S n → G (Vec T n)
      vtr {T = _} ⦃ aG ⦄ f [] = pure ⦃ aG ⦄ []
      vtr {T = _} ⦃ aG ⦄ f (x ∷ xs) = pure ⦃ aG ⦄ _∷_ ⊛ f x ⊛ vtr f xs

  transpose : ∀ {m n X} → Vec (Vec X n) m → Vec (Vec X m) n
  transpose = traverse id

  -- accumulating values in a Monoid stored in a Traversable structure
  crush : ∀ {F X Y} → ⦃ TF : Traversable F ⦄ → ⦃ M : Monoid Y ⦄ → (X → Y) → F X → Y
  crush ⦃ M = M ⦄ = traverse {T = ⊤} ⦃ AG = monoidApplicative ⦃ M ⦄ ⦄

  data Two : Set where
    tt : Two
    ff : Two

  -- conditional operator
  _⟨?⟩_ : ∀ {l} → {P : Two → Set l} → P tt → P ff → (b : Two) → P b
  (t ⟨?⟩ f) tt = t
  (t ⟨?⟩ f) ff = f

  -- define binary sum
  _+_ : Set → Set → Set
  S + T = Σ Two (S ⟨?⟩ T)

