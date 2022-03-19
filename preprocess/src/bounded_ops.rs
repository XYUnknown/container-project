use std::collections::HashMap;
use std::collections::hash_map::Iter;
use crate::types::{Type, TypeVar, Bounds};

type BoundName = String;
type OpName = String;
type OpInfo = (OpName, Type);
pub type BoundedOps = HashMap<BoundName, Vec<OpInfo>>;

pub fn generate_bounded_ops() -> BoundedOps {
    let mut ops = BoundedOps::new();
    let push = ("push".to_string(), Type::Fun(
        Box::new(Type::Con(Box::new("Con".to_string()), 
        Box::new(Type::Var(TypeVar::new("T".to_string()))),
        Box::new(Bounds::from(["Stack".to_string()])))), 
        Box::new(Type::Fun(Box::new(Type::Var(TypeVar::new("T".to_string()))), Box::new(Type::Con(Box::new("Con".to_string()), 
        Box::new(Type::Var(TypeVar::new("T".to_string()))),
        Box::new(Bounds::from(["Stack".to_string()]))))))));
    let pop = ("pop".to_string(), Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
        Box::new(Type::Var(TypeVar::new("T".to_string()))),
        Box::new(Bounds::from(["Stack".to_string()])))), Box::new(Type::Var(TypeVar::new("T".to_string())))));
    ops.insert("Stack".to_string(), vec![push, pop]);
    ops

}