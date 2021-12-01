use crate::parser::{Prog, Block, Spec, Decl, Type, Term, Refinement, Id, spec};
use crate::ctx::{Ctx};
use crate::generator::{readfile};

use std::ops::Deref;

type TypeError = String;
type TypeCheckResult<T> = Result<T, TypeError>;

pub struct TypeChecker {
    global_ctx : Ctx,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            global_ctx: Ctx::new(),
        }
    }

    fn predefined(&mut self) {
        // put for_all_unique_pair into context
        let args0 = vec![Type::Ty(Box::new("T".to_string())), Type::Ty(Box::new("T".to_string()))];
        let binary_fn = Type::Fun(Box::new(args0), Box::new(Type::Bool()));
        let args1 = vec![Type::Ty(Box::new("Con<T>".to_string())), binary_fn];
        self.global_ctx.put("for_all_unique_pair".to_string(), Type::Fun(Box::new(args1), Box::new(Type::Bool())));

        // put neq into context
        let args2 = vec![Type::Ty(Box::new("T".to_string())), Type::Ty(Box::new("T".to_string()))];
        let neq_fn = Type::Fun(Box::new(args2), Box::new(Type::Bool()));
        self.global_ctx.put("neq".to_string(), neq_fn);
    }

    pub fn get_ctx(&self) -> &Ctx {
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
                match self.global_ctx.get_id(id.to_string()) {
                    Some(_) => Err("Duplicate property declaration".to_string()),
                    None => {
                        // check well formedness
                        // it should have type Con<T> -> Bool
                        // otherwise error
                        let mut ctx = Ctx::new();
                        match self.check_term(&mut ctx, term) {
                            Ok(t) => {
                                let desired_prop_type = Type::Fun(Box::new(vec![Type::Ty(Box::new("Con<T>".to_string()))]), Box::new(Type::Bool()));
                                if t == &desired_prop_type {
                                    // TODO: construct property prooerty type and description
                                    let prop_type = Type::PropType(Box::new(desired_prop_type), Box::new(id.to_string()));
                                    self.global_ctx.put(id.to_string(), prop_type);
                                    Ok(())
                                } else {
                                    Err("Not a valid property type".to_string())
                                }
                            },
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
                match self.global_ctx.get_id(id.to_string()) {
                    Some(_) => Err("Duplicate container type declaration".to_string()),
                    None => {
                        // ty has to be Con<T>
                        let con = Type::Ty(Box::new("Con<T>".to_string()));
                        if ty.deref() == &con {
                            // term has to be AppTerm
                            match self.check_ref(r.deref(), vid, con) {
                                Ok(types) => {
                                    let con_type_ref = Type::ConType(Box::new( Type::Ty(Box::new("Con<T>".to_string()))), Box::new(types.to_vec()));
                                    self.global_ctx.put(id.to_string(), con_type_ref);
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

    fn check_ref<'a>(&self, r: &'a Refinement, vid: &'a Box<String>, con: Type) -> Result<Vec<Type>, TypeError> {
        match r {
            Refinement::Prop(term) => {
                match term.deref() {
                    Term::AppTerm(term1, term2) => {
                        // TODO: term has to be evalued to Bool
                        let mut local_ctx = Ctx::new();
                        local_ctx.put(vid.to_string(), con);
                        // self.check_term(..)
                        // evaluate term and construct refined contype
                        match self.check_term_temp(&mut local_ctx, term1, term2) {
                            Ok(ty) => {
                                let types = vec![ty.clone()];
                                Ok(types)
                            },
                            Err(e) => Err(e)
                        }
                    },
                    _ => Err("Not a valid term for refining the type Con<T>".to_string())
                }
            },
            Refinement::AndProps(r1, r2) => {
                match self.check_ref(r1, vid, con.clone()) {
                    Ok(types1) => {
                        match self.check_ref(r2, vid, con.clone()) {
                            Ok(types2) => {
                                Ok([types1, types2].concat())
                            },
                            Err(e) => Err(e)
                        }
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }

    // temp function, remove when term type checking is finished
    // purpose: if well-typed, obtaining the PropType for refining Con<T>
    fn check_term_temp(&self, ctx: &Ctx, term1: &Term, term2: &Term) -> Result<&Type, TypeError> {
        match term2 {
            Term::VarTerm(id) => {
                match ctx.get_id(id.to_string()) {
                    Some(_) => {
                        match term1 {
                            Term::VarTerm(id) => {
                                match self.global_ctx.get_id(id.to_string()) {
                                    Some(ty) => {
                                        match ty {
                                            Type::PropType(_, _) => Ok(ty),
                                            _ => Err(id.to_string() + " does not have a valid property type")
                                        }
                                    },
                                    _ => Err("Undefined variable: ".to_string() + id)
                                }
                            },
                            _ => Err("Should be a varible term".to_string())
                        }
                    },
                    _ => Err("Undefined variable: ".to_string() + id)
                }
            },
            _ => Err("Should be a varible term".to_string())
        }
    }

    pub fn check_term<'a>(&mut self, ctx: &'a mut Ctx, term: &Term) -> Result<&'a Type, TypeError> {
        // TODO: Add proper property type checking later
        let placeholder_type = Type::Fun(Box::new(vec![Type::Ty(Box::new("Con<T>".to_string()))]), Box::new(Type::Bool()));
        ctx.put("placeholder_type".to_string(), placeholder_type);
        match term {
            Term::VarTerm(id) => {
                // Type checking placeholder
                match ctx.get_id("placeholder_type".to_string()) {
                    Some(t) => Ok(t),
                    None => Err("Undefined variable".to_string())
                }
            },
            Term::LambdaTerm(id, term) => {
                // Type checking placeholder
                match ctx.get_id("placeholder_type".to_string()) {
                    Some(t) => Ok(t),
                    None => Err("Undefined variable".to_string())
                }
            },
            Term::AppTerm(term1, term2) => {
                // Type checking placeholder
                match ctx.get_id("placeholder_type".to_string()) {
                    Some(t) => Ok(t),
                    None => Err("Undefined variable".to_string())
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