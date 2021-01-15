module Normal where
  open import Data.Product as Prod using (_×_; _,_; Σ; uncurry)
  open import Data.Unit
  open import Function
  open import Relation.Binary.PropositionalEquality as Eq using (_≡_; subst; refl)
  open import Vec

  -- Normal functors
  record Normal : Set₁ where
    constructor _/_
    field
      Shape : Set
      size : Shape → ℕ
    ⟦_⟧N : Set → Set
    ⟦_⟧N X = Σ Shape λ s → Vec X (size s)
  open Normal public
  infixr 0 _/_

  VecN : ℕ → Normal
  VecN n = ⊤ / pure n

  ListN : Normal
  ListN = ℕ / id

  KN : Set → Normal
  KN A = A / λ _ → zero

  IKN : Normal
  IKN = VecN (suc zero)

  _+N_ : Normal → Normal → Normal
  (Shape₁ / size₁) +N (Shape₂ / size₂) = (Shape₁ + Shape₂) / uncurry (size₁ ⟨?⟩ size₂)

  _×N_ : Normal → Normal → Normal
  (Shape₁ / size₁) ×N (Shape₂ / size₂) = Shape₁ × Shape₂ / uncurry λ x y → size₁ x +ℕ size₂ y

  nInj : ∀ {X} (F G : Normal) → ⟦ F ⟧N X + ⟦ G ⟧N X → ⟦ F +N G ⟧N X
  nInj F G (tt , ShF , xs) = (tt , ShF) , xs
  nInj F G (ff , ShG , xs) = (ff , ShG) , xs

  data _^-1_ {S T : Set} (f : S → T) : T → Set where
    from : (s : S) → f ^-1 f s

  -- nInj is surjective
  nCase : ∀ {X} (F G : Normal) → (s : ⟦ F +N G ⟧N X) → nInj F G ^-1 s
  nCase F G ((tt , ShF) , xs) = from (tt , ShF , xs)
  nCase F G ((ff , ShG) , xs) = from (ff , ShG , xs)

  nOut : ∀ {X} (F G : Normal) → ⟦ F +N G ⟧N X → ⟦ F ⟧N X + ⟦ G ⟧N X
  nOut F G xs            with nCase F G xs
  nOut F G .(nInj F G ys) | from ys = ys

  nPair : ∀ {X} (F G : Normal) → ⟦ F ⟧N X × ⟦ G ⟧N X → ⟦ F ×N G ⟧N X
  nPair F G ((ShF , xsf) , ShG , xsg) = (ShF , ShG) , (xsf ++ xsg)

  instance
    listNMonoid : {X : Set} → Monoid (⟦ ListN ⟧N X)
    listNMonoid = record
      { ε = zero , [] ;
        _∙_ = uncurry λ ShX₁ xs₁ → uncurry λ ShX₂ xs₂ → (ShX₁ +ℕ ShX₂) , (xs₁ ++ xs₂)
      }

    sumMonoid : Monoid ℕ
    sumMonoid = record
      { ε = zero;
        _∙_ = _+ℕ_
      }

  normalTraversable : (F : Normal) → Traversable ⟦ F ⟧N
  normalTraversable F = record
    {traverse = λ ⦃ aG ⦄ f → uncurry λ ShF xs → pure ⦃ aG ⦄ (_,_ ShF) ⊛ traverse f xs}

  -- compositions
  _oN_ : Normal → Normal → Normal
  F oN (ShG / szG) = ⟦ F ⟧N ShG / crush ⦃ normalTraversable F ⦄ szG

  -- Traversable structures have a notion of size induced by the Monoid structure for ℕ
  sizeT : ∀ {F} ⦃ TF : Traversable F ⦄ {X} → F X → ℕ
  sizeT = crush λ _ → suc zero

  -- every Traversable functor has a Normal counterpart
  normalT : ∀ F ⦃ TF : Traversable F ⦄ → Normal
  normalT F = F ⊤ / sizeT

  -- put a Traversable structure into its Normal representation
  shapeT : ∀ {F} ⦃ TF : Traversable F ⦄ {X} → F X → F ⊤
  shapeT = traverse λ _ → tt

  -- define the list of elements, which should have the same length as the size
  one : ∀ {X} → X →  ⟦ ListN ⟧N X
  one x = suc zero , x ∷ []

  contentsT : ∀ {F} ⦃ TF : Traversable F ⦄ {X} → F X → ⟦ ListN ⟧N X
  contentsT = crush one

  -- Normal Morphism
  _→N_ : Normal → Normal → Set
  F →N G = (s : Shape F) → ⟦ G ⟧N (Fin (size F s))

  -- any such thing determines a natural transformation from F to G
  nMorph : ∀ {F G} → F →N G → ∀ {X} → ⟦ F ⟧N X → ⟦ G ⟧N X
  nMorph f (ShF , xs) with f ShF
  nMorph f (ShF , xs) | ShG , ys = ShG , map (proj xs) ys

  -- compute the normal morphism representing a given natural transformation
  morphN : ∀ {F G} → (∀ {X} → ⟦ F ⟧N X → ⟦ G ⟧N X) → F →N G
  morphN f ShF = f (ShF , tabulate id)

  _⊗_ : Normal → Normal → Normal
  (ShF / szF) ⊗ (ShG / szG) = ShF × ShG / uncurry λ x y → szF x ×ℕ szG y

  -- Construct normal morphisms
  swap : (F G : Normal) → (F ⊗ G) →N (G ⊗ F)
  swap F G (ShF , ShG) = (ShG , ShF) ,
    joinVec (size G ShG) (size F ShF)
      (transpose (splitVec (size F ShF) (size G ShG) (tabulate id)))

  subst′ :  forall {k l}{X : Set k}{s t : X} ->
            s ≡ t -> (P : X -> Set l) -> P s -> P t
  subst′ refl P p = p

  drop : (F G : Normal) → (F ⊗ G) →N (F oN G)
  drop F G (ShF , ShG) = (ShF , pure ShG) , subst (Vec _) (lem (size F ShF)) (tabulate id) where
    lem : ∀ n → (n ×ℕ size G ShG) ≡ crush (size G) (vec {n} ShG)
    lem zero = refl
    lem (suc n) rewrite lem n = refl
