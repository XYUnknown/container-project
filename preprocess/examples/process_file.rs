use preprocess::parser::{Term, Decl, spec};
use preprocess::generator::{readfile, process_src, run};
use preprocess::types::{Type, TypeVar, TypeVarGen, TypeScheme};
use preprocess::inference::{TypeEnv, InferenceError};
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// Run a test case
fn test(exp: Term) -> Result<Type, InferenceError> {
    let mut env = TypeEnv::new();
    let mut tvg = TypeVarGen::new();

    let binary_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
    env.insert("for_all_unique_pairs".to_string(),
        TypeScheme {
            vars: Vec::new(),
            ty: Type::Fun(Box::new(Type::Con(Box::new("Con".to_string()), 
                Box::new(Type::T(TypeVar::new("T".to_string()))))), 
                Box::new(Type::Fun(Box::new(binary_fn), Box::new(Type::Bool()))))
            }
        );

    // put neq into context
    let neq_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
    env.insert("neq".to_string(), 
        TypeScheme {
            vars: Vec::new(),
            ty: neq_fn
        }
    );

    // put leq into context
    let leq_fn = Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Fun(Box::new(Type::T(TypeVar::new("T".to_string()))), Box::new(Type::Bool()))));
    env.insert("leq".to_string(), 
        TypeScheme {
            vars: Vec::new(),
            ty: leq_fn
        }
    );
    env.type_inference(&exp, &mut tvg)
}

fn main() {
    //let f = readfile("./spec_code/example.rs".to_string());
    //let f = readfile("./spec_code/example_unique.rs".to_string());
    //println!("{:?}", spec::prog(&f));

    
    /*match spec::term(r#"\c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))"#) {
        Ok(t) => {
            println!("{:?}", test(t));
        },
        Err(e) => println!{"{}", e}
    }*/
    
    //println!("{}", type_of(spec::prog(&f)));
    //println!("{}", process_spec(Vec::<Decl>::new()));
    //process_src("./spec_code/example.rs".to_string());
    println!("{:?}", run("./spec_code/example.rs".to_string(), "example_output.rs".to_string()));
    //println!("{:?}", run("./spec_code/example_unique.rs".to_string(), "example_unique_output.rs".to_string()));
}