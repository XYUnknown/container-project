module type Container = sig

  (** ['a t] is the type of a container whose elements have type ['a]. *)
  type 'a t

  (** [empty] is the empty container. *)
  val empty: 'a t

  (** [is_empty c] is true iff [c] is empty *)
  val is_empty : 'a t -> bool

  (** [size c] is the number of elements in [c].
      [size empty] is [0]. *)
  val size : 'a t -> int

  (** [add c x] is a set containing all the elements of
      [c] as well as element [x]. *)
  val add : 'a t -> 'a -> 'a t

  (** [remove c x] is a set containing all the elements of
      [c] as well as element [x]. *)
  val remove : 'a t -> 'a -> 'a t

  (** [contains c x] is [true] iff [x] is an element of [c]. *)
  val contains : 'a t -> 'a -> bool

end

module ListUniqueContainer : Container = struct
  type 'a t = 'a list
  let empty = []
  let is_empty lst = 
    if lst == [] then true else false
  let contains lst x = List.mem x lst
  let size = List.length
  let add lst x  = 
    if contains lst x then lst else (x :: lst)
  let rec remove lst x  = 
    match lst with
    | [] -> []
    | var :: tail -> 
      if x == var then remove tail x else var :: (remove tail x)
end

module type Set = sig
  include Container
  (**include module type of struct include ListUniqueContainer end*)
  val union: 'a t -> 'a t -> 'a t
end

module ListSet : Set = struct
  type 'a t = 'a list
  let empty = []
  let is_empty lst = 
    if lst == [] then true else false
  let contains lst x = List.mem x lst
  let size = List.length
  let add lst x  = 
    if contains lst x then lst else (x :: lst)
  let rec remove lst x  = 
    match lst with
    | [] -> []
    | var :: tail -> 
      if x == var then remove tail x else var :: (remove tail x)
  let union lst1 lst2 = 
    lst1 @ lst2 |> List.sort_uniq Stdlib.compare
end