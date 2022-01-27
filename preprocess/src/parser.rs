extern crate peg;
use peg::parser;

use std::vec::Vec;

use crate::types::{Name, Type, TypeVar};

pub type Id = String;

#[derive(Clone, Debug)]
pub enum Refinement {
    Prop(Box<Term>),
    AndProps(Box<Refinement>, Box<Refinement>),
}

#[derive(Clone, Debug)]
pub enum Term {
    VarTerm(Box<Id>),
    LambdaTerm(Box<Id>, Box<Term>),
    AppTerm(Box<Term>, Box<Term>),
}

#[derive(Clone, Debug)]
pub enum Decl {
    PropertyDecl(Box<Id>, Box<Term>),
    ConTypeDecl(Box<Id>, (Box<Id>, Box<Type>, Box<Refinement>))
}

impl Decl {
    pub fn is_prop_decl(&self) -> bool {
        match self {
            Decl::PropertyDecl(_, _) => true,
            _ => false
        }
    }

    pub fn is_contype_decl(&self) -> bool {
        match self {
            Decl::ConTypeDecl(_, _) => true,
            _ => false
        }
    }
}

pub type Spec = Vec<Decl>;
pub type Code = String;

#[derive(Clone, Debug)]
pub enum Block {
    SpecBlock(Box<Spec>, usize),
    CodeBlock(Box<Code>, usize)
}

impl Block {
    pub fn is_spec_block(&self) -> bool {
        match self {
            Block::SpecBlock(_, _) => true,
            _ => false
        }
    }

    pub fn is_code_block(&self) -> bool {
        match self {
            Block::CodeBlock(_, _) => true,
            _ => false
        }
    }

    pub fn extract_spec(&self) -> Spec {
        match self {
            Block::SpecBlock(spec, _) => spec.to_vec(),
            _ => Vec::new()
        }
    }

    pub fn extract_code(&self) -> Code {
        match self {
            Block::CodeBlock(code, _) => code.to_string(),
            _ => String::new()
        }
    }
}

pub type Prog = Vec<Block>;

parser!{
pub grammar spec() for str {
    pub rule id() -> Id
        = s:$([ 'a'..='z' | 'A'..='Z' | '_' ]['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '<' | '>' ]*) 
        { s.into() }

    pub rule name() -> Name
        = s:$([ 'a'..='z' | 'A'..='Z' | '_' ]['a'..='z' | 'A'..='Z' | '0'..='9' ]*) 
        { s.into() }
    
    pub rule ty() -> Type
        = precedence! {
            n:name() "<" _ t:ty() _ ">"
            { Type::Con(Box::new(n), Box::new(t)) }
            --
            n:name()
            { Type::T(TypeVar::new(n)) }
        }
  
    pub rule term() -> Term
        = precedence!{
            v:id() { Term::VarTerm(Box::new(v)) }
            --
            "\\" v:id() _ "->" _ t:term() { Term::LambdaTerm(Box::new(v), Box::new(t)) }
            --
            "(" _ t1:term() __ t2:term() _ ")" { Term::AppTerm(Box::new(t1), Box::new(t2)) }
        }
    
    pub rule refinement() -> Refinement
        = precedence!{
            t:term() { Refinement::Prop(Box::new(t)) }
            --
            "(" _ p1:refinement() __ "and" __ p2:refinement() _ ")" { Refinement::AndProps(Box::new(p1), Box::new(p2)) }
        }

    pub rule decl() -> Decl
        = precedence! {
            _ "property" __ p:id() _ "{" _ t:term() _ "}" _ 
            {
                Decl::PropertyDecl(Box::new(p), Box::new(t))
            }
            --
            _ "type" __ t1:id() _ "=" _ "{" _ c:id() _ ":" _ t2:ty() _ "|" _ t:refinement() _ "}" _
            {
                Decl::ConTypeDecl(Box::new(t1), (Box::new(c), Box::new(t2), Box::new(t)))
            }
        }

    pub rule spec() -> Spec
        = _ "/*SPEC*" _ decls: (d:decl() { d }) ** _ _ "*ENDSPEC*/" _
        {
            decls
        }

    pub rule code() -> Code
        = _ "/*CODE*/" c:$((!"/*ENDCODE*/"!"/*SPEC*"!"*ENDSPEC*/"[_])*) "/*ENDCODE*/" _ { c.into() }
    
    pub rule block() -> Block
        = precedence! {
            _ p:position!() s:spec() _ 
            {
                Block::SpecBlock(Box::new(s), p)
            }
            --
            _ p:position!() c:code() _
            {
                Block::CodeBlock(Box::new(c), p)
            }
        }

    pub rule prog() -> Prog
        = _ blocks: (b:block() { b }) ** _ _
        {
            blocks
        }
    
    rule _ = quiet!{[' ' | '\n' | '\t']*}
    rule __ = quiet!{[' ' | '\n' | '\t']+}
            
}}

#[cfg(test)]
mod tests {
    use crate::parser::{Id, Type, spec};

    #[test]
    fn test_id() {
        assert_eq!(spec::id("unique"), Ok(Id::from("unique")));
    }

    #[test]
    fn test_ty() {
        assert!(spec::ty("UniqueCon<T>").is_ok());
    }

    #[test]
    fn test_ty_sim() {
        assert!(spec::ty("T").is_ok());
    }

    #[test]
    fn test_varterm() {
        assert!(spec::term("x").is_ok());
    }

    #[test]
    fn test_lambdaterm() {
        assert!(spec::term(r#"\c -> c"#).is_ok());
    }

    #[test]
    fn test_appterm() {
        assert!(spec::term("(f c)").is_ok());
    }

    #[test]
    fn test_property() {
        assert!(spec::decl(
            r#"property id {
                 \c -> c 
                }"#
            ).is_ok());
    }

    #[test]
    fn test_property_unique() {
        assert!(spec::decl(
            r#"property unique {
                \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
            }"#
            ).is_ok());
    }

    #[test]
    fn test_contype() {
        assert!(spec::decl(
            "type UniqueCon<T> = {c : Con<T> | (unique c)}"
            ).is_ok());
    }

    #[test]
    fn test_spec() {
        assert!(spec::spec( // raw string literal ????
            r#"/*SPEC*
            property unique {
                \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
            }
            type UniqueCon<T> = {c : Con<T> | (unique c)}
            *ENDSPEC*/"#
        ).is_ok())
    }

    #[test]
    fn test_code() {
        assert!(spec::code(
            r#"/*CODE*/fn main () {
                let mut c = UniqueCon::<u32>::new();
                for x in 0..10 {
                    c.insert(x);
                    c.insert(x);
                }
                assert_eq!(c.len(), 10);
            }/*ENDCODE*/"#
        ).is_ok())
    }

    #[test]
    fn test_block_spec() {
        assert!(spec::block(
            r#"/*SPEC*
            property unique {
                \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
            }
            type UniqueCon<T> = {c : Con<T> | (unique c)}
            *ENDSPEC*/"#
        ).is_ok())
    }

    #[test]
    fn test_block_code() {
        assert!(spec::block(
            r#"/*CODE*/fn main () {
                let mut c = UniqueCon::<u32>::new();
                for x in 0..10 {
                    c.insert(x);
                    c.insert(x);
                }
                assert_eq!(c.len(), 10);
            }/*ENDCODE*/"#
        ).is_ok())
    }

    #[test]
    fn test_prog() {
        assert!(spec::prog(
            r#"
            /*SPEC*
            property unique {
                \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
            }
            type UniqueCon<T> = {c : Con<T> | (unique c) }
            *ENDSPEC*/

            /*CODE*/
            fn main () {
                let mut c = UniqueCon::<u32>::new();
                for x in 0..10 {
                    c.insert(x);
                    c.insert(x);
                }
                assert_eq!(c.len(), 10);
            }
            /*ENDCODE*/

            /*SPEC*
            property unique {
                \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
            }
            type UniqueCon<T> = {c : Con<T> | (unique c) }
            *ENDSPEC*/
            
            /*CODE*/
            fn main () {
                let mut c = UniqueCon::<u32>::new();
                for x in 0..10 {
                    c.insert(x);
                    c.insert(x);
                }
                assert_eq!(c.len(), 10);
            }
            /*ENDCODE*/"#
        ).is_ok())
    }
}