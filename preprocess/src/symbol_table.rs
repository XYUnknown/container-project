use std::collections::HashMap;
use crate::parser::{Id, Type};

#[derive(Clone, Debug)]
pub struct SymbolTable {
    globals: HashMap<Id, Type>,
    locals: Option<HashMap<Id, Type>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            globals: HashMap::new(),
            locals: None
        }
    }

    pub fn enter_loc(&mut self) {
        self.locals = Some(HashMap::new());
    }

    pub fn exit_loc(&mut self) {
        self.locals = None;
    }

    pub fn put(&mut self, id: Id, ty: Type) -> bool {
        match &mut self.locals {
            Some(t) => {
                if t.contains_key(&id) {
                    false
                } else {
                    t.insert(id, ty);
                    true
                }
            },
            None => {
                if self.globals.contains_key(&id) {
                    false
                } else {
                    self.globals.insert(id, ty);
                    true
                }
            }
        }
    }

    pub fn get_id(&self, id: Id) -> Option<&Type> {
        match &self.locals {
            Some(t) => {
                if t.contains_key(&id) {
                    t.get(&id)
                } else {
                    self.globals.get(&id)
                }
            },
            None => {
                self.globals.get(&id)
            }
        }
    }

    pub fn get_id_loc(&self, id: Id) -> Option<&Type> {
        match &self.locals {
            Some(t) => t.get(&id),
            None => None
        }
    }

    fn global_sz(&self) -> usize {
        self.globals.len()
    }

    fn local_sz(&self) -> Option<usize> {
        match &self.locals {
            Some(t) => Some(t.len()),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::{SymbolTable};
    use crate::parser::{Type};
    #[test]
    fn test_table_insert() {
        let mut t = SymbolTable::new();
        assert!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))));
        assert_eq!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))), false);
        assert_eq!(t.global_sz(), 1);
        assert_eq!(t.local_sz(), None);
    }

    #[test]
    fn test_table_insert_loc() {
        let mut t = SymbolTable::new();
        t.enter_loc();
        assert!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))));
        assert_eq!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))), false);
        assert_eq!(t.global_sz(), 0);
        assert_eq!(t.local_sz(), Some(1));
    }

    #[test]
    fn test_table_get() {
        let mut t = SymbolTable::new();
        t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string())));
        assert_eq!(t.get_id("c".to_string()), Some(&Type::Ty(Box::new("Con<T>".to_string()))));
    }

    #[test]
    fn test_table_get_loc() {
        let mut t = SymbolTable::new();
        t.enter_loc();
        t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string())));
        assert_eq!(t.get_id_loc("c".to_string()), Some(&Type::Ty(Box::new("Con<T>".to_string()))));
    }
}