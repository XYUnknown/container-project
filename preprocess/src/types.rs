use std::borrow::Borrow;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::result;

pub type Name = String;

// traits
pub type Bounds = Vec<Name>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Type {
    Bool(),
    Int(),
    Var(TypeVar),
    Con(Box<Name>, Box<Type>, Box<Bounds>),
    Fun(Box<Type>, Box<Type>)
}

impl Type {
    pub fn is_var(&self) -> bool {
        match self {
            Type::Var(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Type::Bool() => true,
            _ => false
        }
    }

    pub fn get_con_elem(&self) -> Option<(String, String)> {
        match self {
            Type::Con(n, t, _) => Some((n.to_string(), t.to_string())),
            _ => None
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Bool() => "bool".to_string(),
            Type::Int() => "int".to_string(),
            Type::Var(tv) => tv.to_string(),
            Type::Con(n, t, bounds) => n.to_string() + "<" + &t.to_string() + ">" + " <: (" + &bounds.join(", ") + ")",
            Type::Fun(t1, t2) => t1.to_string() + "->" + &t2.to_string(),
        }
    }
}

pub type UnificationError = String;

trait Union {
    fn union(&self, other: &Self) -> Self;
}

impl<K, V> Union for HashMap<K, V>
    where K: Clone + Eq + Hash,
          V: Clone
{
    fn union(&self, other: &Self) -> Self {
        let mut res = self.clone();
        for (key, value) in other {
            res.entry(key.clone()).or_insert(value.clone());
        }
        res
    }
}

impl Type {
    // Most general unifier
    pub fn mgu(&self, other: &Type) -> Result<Subst, UnificationError> {
        match (self, other) {
            // Unify function type
            (&Type::Fun(ref in1, ref out1), &Type::Fun(ref in2, ref out2)) => {
                let sub1 = in1.mgu(in2)?;
                let sub2 = out1.apply(&sub1).mgu(&out2.apply(&sub1))?;
                Ok(sub1.compose(&sub2))
            }

            // Unify con type
            (&Type::Con(ref n1, ref t1,  _), &Type::Con(ref n2, ref t2, _)) => {
                if n1.to_string() != n2.to_string() {
                    Err("Cannot unify two different container".to_string())
                } else {
                    t1.mgu(t2)
                }
            }

            // Type variable biding
            (&Type::Var(ref v), t) => v.bind(t),
            (t, &Type::Var(ref v)) => v.bind(t),

            // Unify primitives
            (&Type::Int(), &Type::Int()) | (&Type::Bool(), &Type::Bool()) => {
                Ok(Subst::new())
            }

            // Otherwise, the types cannot be unified.
            (t1, t2) => Err("types do not unify".to_string()),
        }
    }
}

// A substitution is a mapping from type variables to types.
#[derive(Clone, Debug)]
pub struct Subst(HashMap<TypeVar, Type>);

impl Deref for Subst {
    type Target = HashMap<TypeVar, Type>;
    fn deref(&self) -> &HashMap<TypeVar, Type> {
        &self.0
    }
}
impl DerefMut for Subst {
    fn deref_mut(&mut self) -> &mut HashMap<TypeVar, Type> {
        &mut self.0
    }
}

impl Subst {
    pub fn new() -> Subst {
        Subst(HashMap::new())
    }

    // composing substitutions
    pub fn compose(&self, other: &Subst) -> Subst {
        Subst(self.union(&other.iter()
                               .map(|(k, v)| (k.clone(), v.apply(self)))
                               .collect()))
    }
}

// Fresh variable generator
pub struct TypeVarGen {
    supply: usize,
}

impl TypeVarGen {
    pub fn new() -> TypeVarGen {
        TypeVarGen { supply: 0 }
    }
    pub fn next(&mut self) -> TypeVar {
        let name = "#T".to_owned() + &self.supply.to_string();
        let v = TypeVar::new(name);
        self.supply += 1;
        v
    }
}

// Type variables/type names
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeVar {
    name: Name,
}

impl TypeVar {
    pub fn new(s: Name) -> TypeVar {
        TypeVar{name: s}
    }
    /// Attempt to bind a type variable to a type, returning an appropriate substitution.
    fn bind(&self, ty: &Type) -> Result<Subst, UnificationError> {
        // Binding to itself
        if let &Type::Var(ref u) = ty {
            if u == self {
                return Ok(Subst::new());
            }
        }

        // Occurance check
        if ty.ftv().contains(self) {
            return Err("occur check fails".to_string());
        }

        let mut s = Subst::new();
        s.insert(self.clone(), ty.clone());
        Ok(s)
    }
}


impl ToString for TypeVar {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

pub trait Types {
    fn ftv(&self) -> HashSet<TypeVar>;
    fn apply(&self, s: &Subst) -> Self;
}

impl<'a, T> Types for Vec<T>
    where T: Types
{
    // Free type variables
    fn ftv(&self) -> HashSet<TypeVar> {
        self.iter()
            .map(|x| x.ftv())
            .fold(HashSet::new(), |set, x| set.union(&x).cloned().collect())
    }

    // Apply a substitution to a vector of types
    fn apply(&self, s: &Subst) -> Vec<T> {
        self.iter().map(|x| x.apply(s)).collect()
    }
}

impl Types for Type {
    fn ftv(&self) -> HashSet<TypeVar> {
        match self {
            &Type::Var(ref s) => [s.clone()].iter().cloned().collect(),
            &Type::Int() | &Type::Bool() => HashSet::new(),
            &Type::Fun(ref i, ref o) => i.ftv().union(&o.ftv()).cloned().collect(),
            &Type::Con(_, ref s, _) => s.ftv().union(&HashSet::new()).cloned().collect(),
        }
    }

    // apply substitution
    fn apply(&self, s: &Subst) -> Type {
        match self {
            &Type::Var(ref n) => s.get(n).cloned().unwrap_or(self.clone()),
            &Type::Fun(ref t1, ref t2) => Type::Fun(Box::new(t1.apply(s)), Box::new(t2.apply(s))),
            &Type::Con(ref n, ref t, ref bounds) => Type::Con(Box::new(n.to_string()), Box::new(t.apply(s)), bounds.clone()),
            _ => self.clone(),
        }
    }
}

// A type scheme is a type with an extra piece of information attached, to constraint the inference
#[derive(Clone, Debug)]
pub struct TypeScheme {
    pub vars: Vec<TypeVar>,
    pub ty: Type,
}

impl Types for TypeScheme {
    fn ftv(&self) -> HashSet<TypeVar> {
        self.ty
            .ftv()
            .difference(&self.vars.iter().cloned().collect())
            .cloned()
            .collect()
    }

    fn apply(&self, s: &Subst) -> TypeScheme {
        TypeScheme {
            vars: self.vars.clone(),
            ty: {
                let mut sub = s.clone();
                for var in &self.vars {
                    sub.remove(var);
                }
                self.ty.apply(&sub)
            },
        }
    }
}

impl TypeScheme {
    /// Instantiates a typescheme into a type. 
    pub fn instantiate(&self, tvg: &mut TypeVarGen) -> Type {
        let newvars = self.vars.iter().map(|_| Type::Var(tvg.next()));
        self.ty.apply(&Subst(self.vars
                                 .iter()
                                 .cloned()
                                 .zip(newvars)
                                 .collect()))
    }
}