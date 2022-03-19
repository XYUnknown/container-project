use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::result;

use crate::parser::{Id, Term};
use crate::types::{Name, Type, TypeVar, Types, TypeVarGen, Subst, TypeScheme};
use crate::bounded_ops::{BoundedOps, generate_bounded_ops};

/// A type environment
#[derive(Clone, Debug)]
pub struct TypeEnv(HashMap<Id, TypeScheme>);

impl Deref for TypeEnv {
    type Target = HashMap<Id, TypeScheme>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TypeEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Types for TypeEnv {
    fn ftv(&self) -> HashSet<TypeVar> {
        self.values().map(|x| x.clone()).collect::<Vec<TypeScheme>>().ftv()
    }

    fn apply(&self, s: &Subst) -> TypeEnv {
        TypeEnv(self.iter()
                    .map(|(k, v)| (k.clone(), v.apply(s)))
                    .collect())
    }
}

impl TypeEnv {
    pub fn new() -> TypeEnv {
        TypeEnv(HashMap::new())
    }

    fn generalise(&self, ty: &Type) -> TypeScheme {
        TypeScheme {
            vars: ty.ftv().difference(&self.ftv()).cloned().collect(),
            ty: ty.clone(),
        }
    }

    // Main type inference algorithm
    fn ti(&self, term: &Term, tvg: &mut TypeVarGen) -> Result<(Subst, Type), InferenceError> {
        // Get types of operations defined in traits
        let bounded_ops = generate_bounded_ops();
        let (s, t) = (match term {
            // Infer literal: currently only boolean
            Term::LitTerm(_) => {
                Ok((Subst::new(), Type::Bool()))
            }
            // Infer variable
            Term::VarTerm(v) => {
                match self.get(&v.to_string()) {
                    Some(s) => Ok((Subst::new(), s.instantiate(tvg))),
                    None => Err("unbound variable".to_string() + " " + &v.to_string()),
                }
            }
            // Infer abstraction
            Term::LambdaTerm((n, bounds), ref e) => {
                let mut tv = Type::Var(tvg.next());
                let mut env = self.clone();
                if (!bounds.is_empty()) {
                    tv = Type::Con(Box::new("Con".to_string()), Box::new(tv), bounds.clone());
                    for b in bounds.iter() {
                        if bounded_ops.contains_key(b) {
                            let ops_info = bounded_ops.get(b).unwrap();
                            for (op_name, op_ty) in ops_info {
                                env.insert(
                                    op_name.to_string(), 
                                    TypeScheme {
                                        vars: Vec::new(),
                                        ty: op_ty.clone(),
                                    }
                                );
                            }
                        }
                    }
                }
                env.remove(&n.to_string());

                env.insert(
                    n.to_string(), 
                    TypeScheme {
                        vars: Vec::new(),
                        ty: tv.clone(),
                    }
                );
                let (s1, t1) = env.ti(e, tvg)?;
                let result_ty = Type::Fun(Box::new(tv.apply(&s1)), Box::new(t1));
                Ok((s1.clone(), result_ty))
            }
            // Infer application
            Term::AppTerm(ref e1, ref e2) => {
                let (s1, t1) = self.ti(e1, tvg)?;
                let (s2, t2) = self.apply(&s1).ti(e2, tvg)?;
                let tv = Type::Var(tvg.next());
                let s3 = t1.apply(&s2).mgu(&Type::Fun(Box::new(t2), Box::new(tv.clone())))?;
                Ok((s3.compose(&s2.compose(&s1)), tv.apply(&s3)))
            }
        })?;
        Ok((s, t))
    }

    // perform type inference on term
    pub fn type_inference(&self, term: &Term, tvg: &mut TypeVarGen) -> Result<Type, InferenceError> {
        let (s, t) = self.ti(term, tvg)?;
        Ok(t.apply(&s))
    }
}

pub type InferenceError = String;