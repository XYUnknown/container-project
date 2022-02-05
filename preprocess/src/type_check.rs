use crate::parser::{Prog, Block, Spec, Decl, Term, Refinement, Id, spec};
use crate::inference::{TypeEnv};
use crate::generator::{readfile};
use crate::types::{Type, TypeVar, TypeScheme, TypeVarGen};

use std::ops::Deref;

type TypeError = String;

pub struct TypeChecker {
    global_ctx : TypeEnv,
    tvg: TypeVarGen
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            global_ctx: TypeEnv::new(),
            tvg: TypeVarGen::new()
        }
    }

    fn predefined(&mut self) {
        // put for_all_unique_pair into context
        let binary_fn1 = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
        self.global_ctx.insert("for-all-unique-pairs".to_string(),
            TypeScheme {
                vars: Vec::new(),
                ty: Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
                    Box::new(Type::T(TypeVar::new("T".to_string()))))), 
                    Box::new(Type::Fun(Box::new(binary_fn1), Box::new(Type::Bool()))))
                }
            );

        // put for_all_unique_pair into context
        let binary_fn2 = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
        self.global_ctx.insert("for-all-consecutive-pairs".to_string(),
            TypeScheme {
                vars: Vec::new(),
                ty: Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
                    Box::new(Type::T(TypeVar::new("T".to_string()))))), 
                    Box::new(Type::Fun(Box::new(binary_fn2), Box::new(Type::Bool()))))
                }
            );
        
        let unary_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()));
        self.global_ctx.insert("for-all-elems".to_string(),
            TypeScheme {
                vars: Vec::new(),
                ty: Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
                    Box::new(Type::T(TypeVar::new("T".to_string()))))), 
                    Box::new(Type::Fun(Box::new(unary_fn), Box::new(Type::Bool()))))
                }
            );

        // put neq into context
        let neq_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
        self.global_ctx.insert("neq".to_string(), 
            TypeScheme {
                vars: Vec::new(),
                ty: neq_fn
            }
        );

        // put leq into context
        let leq_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
        self.global_ctx.insert("leq?".to_string(), 
            TypeScheme {
                vars: Vec::new(),
                ty: leq_fn
            }
        );

        let unique_count_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), 
                                Box::new(Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
                                Box::new(Type::T(TypeVar::new("T".to_string()))))), Box::new(Type::Bool()))));
        self.global_ctx.insert("unique-count?".to_string(), 
            TypeScheme {
                vars: Vec::new(),
                ty: unique_count_fn
            }
        );
    }

    pub fn get_ctx(&self) -> &TypeEnv {
        &self.global_ctx
    }

    pub fn check_prog(&mut self, prog: Prog) -> Result<(), TypeError> {
        let specs: Vec<Spec> = 
            prog.iter()
            .filter(| block | block.is_spec_block())
            .map(| block | block.extract_spec())
            .collect();
        self.predefined();
        self.check_specs(specs)
    }

    pub fn check_specs(&mut self, specs: Vec<Spec>) -> Result<(), TypeError> {
        let concat_specs = specs.concat();
        let prop_decls: Vec<&Decl> =
            concat_specs.iter()
            .filter(| decl | decl.is_prop_decl())
            .collect();
        let contype_decls: Vec<&Decl> =
            concat_specs.iter()
            .filter(| decl | decl.is_contype_decl())
            .collect();
        match self.check_prop_decls(prop_decls) {
            Ok(_) => self.check_contype_decls(contype_decls),
            Err(e) => Err(e)
        }
    }

    pub fn check_prop_decls(&mut self, decls: Vec<&Decl>) -> Result<(), TypeError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.check_prop_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn check_prop_decl(&mut self, decl: &Decl) -> Result<(), TypeError> {
        match decl {
            Decl::PropertyDecl(id, term) => {
                // Duplicate property decl checking
                match self.global_ctx.get(&id.to_string()) {
                    Some(_) => Err("Duplicate property declaration".to_string()),
                    None => {
                        // check well formedness
                        match self.global_ctx.type_inference(term, &mut self.tvg) {
                            Ok(ty) => {
                                // it should have type Con<T> -> Bool
                                match ty {
                                    Type::Fun(ref t1, ref t2) => {
                                        match (t1.deref(), t2.deref()) {
                                            (Type::Con(n, t), Type::Bool()) => {
                                                if n.to_string() == "Con".to_string() {
                                                    self.global_ctx.insert(
                                                        id.to_string(),
                                                        TypeScheme {
                                                            vars: Vec::new(),
                                                            ty: ty
                                                        }
                                                    );
                                                    Ok(())
                                                } else {
                                                    Err("Not a valid property decl: input does not have basic container type Con<T>".to_string())
                                                }
                                            },
                                            _ => Err("Not a valid property decl: should have type Con<T> -> Bool".to_string())
                                        }
                                    },
                                    _ => Err("Not a valid property decl: should have type Con<T> -> Bool".to_string())
                                }
                            }
                            Err(e) => Err(e)
                        }
                    },
                }
            },
            _ => Err("Not a valid property declaration".to_string())
        }
    }

    pub fn check_contype_decls(&mut self, decls: Vec<&Decl>) -> Result<(), TypeError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.check_contype_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn check_contype_decl(&mut self, decl: &Decl) -> Result<(), TypeError> {
        match decl {
            Decl::ConTypeDecl(id, (vid, ty, r)) => {
                // Duplicate container type decl checking
                match self.global_ctx.get(&id.to_string()) {
                    Some(_) => Err("Duplicate container type declaration".to_string()),
                    None => {
                        // ty has to be Con<T>
                        let con = Type::Con(Box::new("Con".to_string()), Box::new(Type::T(TypeVar::new("T".to_string()))));
                        if ty.deref() == &con {
                            let mut local_ctx = self.global_ctx.clone();
                            local_ctx.insert(vid.to_string(),
                                TypeScheme {
                                    vars: Vec::new(),
                                    ty: con
                                }
                            );
                            match self.check_ref(&mut local_ctx, r) {
                                Ok(_) => {
                                    self.global_ctx.insert(id.to_string(), 
                                        TypeScheme{
                                            vars: Vec::new(),
                                            ty: Type::Con(Box::new(id.to_string()), Box::new(Type::T(TypeVar::new("T".to_string()))))
                                        }
                                    );
                                    Ok(())
                                },
                                Err(e) => Err(e)
                            }
                        } else {
                            Err("The base type should be a basic container Con<T>".to_string())
                        }
                    },
                }
            },
            _ => Err("Not a valid container type declaration".to_string())
        }
    }

    pub fn check_ref(&mut self, ctx: &mut TypeEnv, r: &Refinement) -> Result<(), TypeError> {
        match r {
            Refinement::Prop(t) => {
                match ctx.type_inference(t.deref(), &mut self.tvg) {
                    Ok(t) => {
                        // t has to be boolean
                        if t.is_bool() {
                            Ok(())
                        } else {
                            Err("The refinement has evaluates to a Bool type.".to_string())
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            Refinement::AndProps(r1, r2) => {
                match self.check_ref(ctx, r1) {
                    Ok(_) => {
                        match self.check_ref(ctx, r2) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e)
                        }
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }
}


// Helper functions of testing
// TODO; restructure them
pub fn check_prop_decl() -> Result<(), TypeError> {
    let mut tc = TypeChecker::new();
    let f = readfile("./spec_code/example.rs".to_string());
    match spec::prog(&f) {
        Ok(prog) => tc.check_prog(prog),
        Err(e) => Err(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::type_check::{TypeChecker, check_prop_decl};

    #[test]
    fn test_dup_prop_decl() {
        assert!(check_prop_decl().is_ok());
    }

}