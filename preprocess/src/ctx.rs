use std::collections::HashMap;
use std::collections::hash_map::Iter;

use crate::parser::{Id, Type};

#[derive(Clone, Debug)]
pub struct Ctx {
    ctx: HashMap<Id, Type>,
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx {
            ctx: HashMap::new(),
        }
    }

    pub fn put(&mut self, id: Id, ty: Type) -> bool {
        if self.ctx.contains_key(&id) {
            false
        } else {
            self.ctx.insert(id, ty);
            true
        }
    }

    pub fn get_id(&self, id: Id) -> Option<&Type> {
        self.ctx.get(&id)
    }

    pub fn iter(&self) -> Iter<'_, Id, Type> {
        self.ctx.iter()
    }

    fn sz(&self) -> usize {
        self.ctx.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::ctx::{Ctx};
    use crate::parser::{Type};
    #[test]
    fn test_table_insert() {
        let mut t = Ctx::new();
        assert!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))));
        assert_eq!(t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string()))), false);
        assert_eq!(t.sz(), 1);
    }

    #[test]
    fn test_table_get() {
        let mut t = Ctx::new();
        t.put("c".to_string(), Type::Ty(Box::new("Con<T>".to_string())));
        assert_eq!(t.get_id("c".to_string()), Some(&Type::Ty(Box::new("Con<T>".to_string()))));
    }
}