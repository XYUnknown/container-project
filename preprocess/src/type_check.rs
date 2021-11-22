use crate::parser::{Prog, Block, Spec, Decl, Type, spec};
use crate::symbol_table::{SymbolTable};
use crate::generator::{readfile};

type TypeError = String;
type TypeCheckResult<T> = Result<T, TypeError>;

pub struct TypeChecker {
    table : SymbolTable,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            table: SymbolTable::new(),
        }
    }

    pub fn check_prog(&mut self, prog: Prog) -> Result<(), TypeError> {
        // TODO: enter predefined functions into the symbol table
        let specs: Vec<Spec> = 
            prog.iter()
            .filter(| block | block.is_spec_block())
            .map(| block | block.extract_spec())
            .collect();
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
        // check well formedness
        // type checking
        match decl {
            Decl::PropertyDecl(id, term) => {
                // Duplicate property decl checking
                match self.table.get_id(id.to_string()) {
                    Some(_) => Err("Duplicate property declaration".to_string()),
                    None => {
                        self.table.put(id.to_string(), Type::Bool()); // Should be: put(id, check_term(term))
                        Ok(())
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

    pub fn check_term() {
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