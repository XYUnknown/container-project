use crate::parser::{Prog, Block, Spec, Decl, Type, Term, spec};
use crate::ctx::{Ctx};
use crate::generator::{readfile};

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
        // No duplication
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
                                    self.global_ctx.put(id.to_string(), desired_prop_type);
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

    pub fn check_contype_decls(&self, decls: Vec<&Decl>) -> Result<(), TypeError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.check_contype_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn check_contype_decl(&self, decl: &Decl) -> Result<(), TypeError> {
        Ok(())
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
        assert!(check_prop_decl().is_err());
    }

}