use crate::parser::{Prog, Block, Spec, Decl, spec};
use crate::symbol_table::{SymbolTable};

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

    pub fn check_prog(&self, prog: Prog) -> Result<(), TypeError> {
        // TODO: enter predefined functions into the symbol table
        let specs: Vec<Spec> = 
            prog.iter()
            .filter(| block | block.is_spec_block())
            .map(| block | block.extract_spec())
            .collect();
        self.check_specs(specs)
    }

    pub fn check_specs(&self, specs: Vec<Spec>) -> Result<(), TypeError> {
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

    pub fn check_prop_decls(&self, decls: Vec<&Decl>) -> Result<(), TypeError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.check_prop_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn check_prop_decl(&self, decl: &Decl) -> Result<(), TypeError> {
        Ok(())
    }

    pub fn check_contype_decls(&self, decls: Vec<&Decl>) -> Result<(), TypeError> {
        Ok(())
    }

    pub fn check_contype_decl() {

    }

    pub fn check_term() {

    }
}