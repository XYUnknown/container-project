extern crate peg;
use peg::parser;

use std::vec::Vec;

type Id = String;
type Type = String;

#[derive(Clone, Debug)]
pub enum Term {
    VarTerm(Box<Id>),
    LambdaTerm(Box<Id>, Box<Term>),
    AppTerm(Box<Term>, Box<Term>),
}

#[derive(Clone, Debug)]
pub enum Decl {
    PropertyDecl(Box<Id>, Box<Term>),
    ConTypeDecl(Box<Type>, (Box<Id>, Box<Type>, Box<Term>))
}

type Spec = Vec<Decl>; 

parser!{
grammar spec() for str {
    pub rule id() -> Id
        = s:$([ 'a'..='z' | 'A'..='Z' | '_' ]['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) 
        { s.into() }
    
    pub rule ty() -> Type
        = s:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' | '<' | '>' ]*) 
        { s.into() }
  
    pub rule term() -> Term
        = precedence!{
            _ v:id() _ { Term::VarTerm(Box::new(v)) }
            --
            _ "\\" v:id() _ "->" _ t:term() _ { Term::LambdaTerm(Box::new(v), Box::new(t)) }
            --
            _ "\\" v:id() _ "->" _ "(" _ t:term() _ ")" _ { Term::LambdaTerm(Box::new(v), Box::new(t)) }
            --
            _ "(" _ t1:term() _ ")" _ t2:id() _ { Term::AppTerm(Box::new(t1), Box::new(Term::VarTerm(Box::new(t2)))) }
            --
            _ "(" _ t1:term() _ ")" _ "(" _ t2:term() _ ")" _ { Term::AppTerm(Box::new(t1), Box::new(t2)) }
        }

    pub rule decl() -> Decl
        = precedence! {
            _ "property" __ p:id() _ "{" _ t:term() _ "}" _ 
            {
                Decl::PropertyDecl(Box::new(p), Box::new(t))
            }
            --
            _ "type" __ t1:ty() _ "=" _ "{" _ c:id() _ ":" _ t2:ty() _ "|" _ t:term() _ "}" _
            {
                Decl::ConTypeDecl(Box::new(t1), (Box::new(c), Box::new(t2), Box::new(t)))
            }
        }

    pub rule spec() -> Spec
        = _ "/*SPEC*" _ decls: (d:decl() { d }) ** _ _ "*ENDSPEC*/" {
            decls
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
        assert_eq!(spec::ty("UniqueCon<T>"), Ok(Type::from("UniqueCon<T>")));
    }

    #[test]
    fn test_varterm() {
        assert!(spec::term("x").is_ok());
    }

    #[test]
    fn test_lambdaterm() {
        assert!(spec::term("\\c -> c").is_ok());
    }

    #[test]
    fn test_appterm() {
        assert!(spec::term("(f)(c)").is_ok());
    }

    #[test]
    fn test_appterm_1() {
        assert!(spec::term("(f) c").is_ok());
    }

    #[test]
    fn test_property() {
        assert!(spec::decl(
            "property id {
                 \\c -> c 
                }"
            ).is_ok());
    }

    #[test]
    fn test_property_unique() {
        assert!(spec::decl(
            "property unique {
                \\c -> ((for_all_unique_pairs) c) (\\a -> (\\b -> ((neq) a) b))
            }"
            ).is_ok());
    }

    #[test]
    fn test_contype() {
        assert!(spec::decl(
            "type UniqueCon<T> = {c : Con<T> | (unique) c}"
            ).is_ok());
    }

    #[test]
    fn test_spec() {
        assert!(spec::spec(
            "/*SPEC*
            property unique {
                \\c -> ((for_all_unique_pairs) c) (\\a -> (\\b -> ((neq) a) b))
            }
            type UniqueCon<T> = {c : Con<T> | (unique) c}
            *ENDSPEC*/"
        ).is_ok())
    }
}