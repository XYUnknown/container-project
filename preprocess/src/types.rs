
pub type Name = String;
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Type {
    Bool(),
    Int(),
    T(Box<Name>),
    Con(Box<Name>, Box<Type>),
    Fun(Box<Vec<Type>>, Box<Type>)
}