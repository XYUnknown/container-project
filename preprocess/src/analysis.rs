use crate::description::{Tag, InforMap};
use crate::parser::{Prog, Block, Spec, Decl, Term, Refinement, Id, spec};
use crate::spec_map::{PropSpecs};

use std::ops::Deref;
use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

type AnalyserError = String;
const LANGDECL: &str = "#lang rosette\n";
const REQUIRE: &str = "(require \"../combinators.rkt\")\n";
const EXTRAREQUIRE: &str = "(require \"../gen_lib_spec/ops.rkt\")\n";
const GENPATH: &str = "./racket_specs/gen_prop_spec/";

// length can be adjusted
// set to 5 to speed up testing
const LISTMODEL: &str =
r#"
(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 5) len))
"#;

fn gen_symbolic(n: &str) -> String {
    format!(
r#"
(define-symbolic {n} integer?)
"#)
}

fn gen_symbolics(symbolics: &Vec<String>) -> String {
    let provide = symbolics.join(" ");
    let mut code = String::new();
    for s in symbolics.iter () {
        code = code + &gen_symbolic(s);
    }
    let provide = format!(
r#"
(provide {provide} ls)
"#); 
    code = code + &provide;
    code
}

pub struct Analyser {
    ctx: InforMap,
    prop_specs: PropSpecs,
}

impl Analyser {
    pub fn new() -> Analyser {
        Analyser {
            ctx: InforMap::new(),
            prop_specs: PropSpecs::new()
        }
    }

    pub fn get_ctx(&self) -> &InforMap {
        &self.ctx
    }

    pub fn get_prop_specs(&self) -> &PropSpecs {
        &self.prop_specs
    }


    pub fn analyse_prog(&mut self, prog: Prog) -> Result<(), AnalyserError> {
        let specs: Vec<Spec> = 
            prog.iter()
            .filter(| block | block.is_spec_block())
            .map(| block | block.extract_spec())
            .collect();
        self.analyse_specs(specs)
    }

    pub fn analyse_specs(&mut self, specs: Vec<Spec>) -> Result<(), AnalyserError> {
        let concat_specs = specs.concat();
        let prop_decls: Vec<&Decl> =
            concat_specs.iter()
            .filter(| decl | decl.is_prop_decl())
            .collect();
        let contype_decls: Vec<&Decl> =
            concat_specs.iter()
            .filter(| decl | decl.is_contype_decl())
            .collect();
        match self.analyse_prop_decls(prop_decls) {
            Ok(_) => match self.analyse_contype_decls(contype_decls.clone()) {
                Ok(_) => self.analyse_bound_decls(contype_decls),
                Err(e) => Err(e)
            }
            Err(e) => Err(e)
        }
    }

    pub fn analyse_prop_decls(&mut self, decls: Vec<&Decl>) -> Result<(), AnalyserError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.analyse_prop_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn analyse_prop_decl(&mut self, decl: &Decl) -> Result<(), AnalyserError> {
        match decl {
            Decl::PropertyDecl((id, _), term) => {
                let mut mterm = term.clone();
                let mut cdr_added = Vec::<String>::new();
                let mut symbolics =  Vec::<String>::new();
                let code =  "(define ".to_string() + id + " " + &self.analyse_term(&mut mterm, true, false, &mut cdr_added, &mut symbolics) + ")\n" + "(provide " + id + ")";
                let filename = id.to_string() + ".rkt";
                let mut symbolics_provided = gen_symbolics(&vec!["n".to_string()]);
                if (symbolics.len() != 0) {
                    symbolics_provided = gen_symbolics(&symbolics);
                } 
                self.write_prop_spec_file(filename.clone(), code, symbolics_provided);
                let prop_tag = Tag::Prop(Box::new(id.to_string()));
                self.ctx.put(id.to_string(), prop_tag);
                if (symbolics.len() == 0) {
                    self.prop_specs.insert(id.to_string(), (filename, vec!["n".to_string()]));
                } else {
                    self.prop_specs.insert(id.to_string(), (filename, symbolics));
                }
                Ok(())
            },
            _ => Err("Not a valid property declaration".to_string())
        }
    }

    pub fn analyse_bound_decls(&mut self, decls: Vec<&Decl>) -> Result<(), AnalyserError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.analyse_bound_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn analyse_bound_decl(&mut self, decl: &Decl) -> Result<(), AnalyserError> {
        match decl {
            Decl::ConTypeDecl(con_ty, (_, ins, tags)) => {
                let (c, t) = con_ty.get_con_elem().unwrap();
                let mut name = c.clone() + "Trait";
                let bound_tag = Tag::Bound((c.clone(), t), Box::new(ins.clone().into_iter().collect::<Vec<String>>()));
                let immut_ctx = self.ctx.clone();
                // prevent generating existing name
                let mut i: usize = 0;
                while immut_ctx.contains(&name) {
                    name = name + &i.to_string();
                    i = i+1;
                }
                let con_tag = immut_ctx.get_id(c.clone()).unwrap();
                match con_tag {
                    Tag::Con(elem_ty, _, tags) => {
                        self.ctx.update(c.clone(), Tag::Con(elem_ty.to_string(), name.clone(), Box::new(tags.to_vec())));
                    },
                    _ => { 
                        return Err("Not a valid container declaration.".to_string());
                    }
                }
                self.ctx.put(name, bound_tag);
                Ok(())
            },
            _ => Err("Not a valid bound declaration".to_string())
        }
    }

    pub fn analyse_contype_decls(&mut self, decls: Vec<&Decl>) -> Result<(), AnalyserError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.analyse_contype_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn analyse_contype_decl(&mut self, decl: &Decl) -> Result<(), AnalyserError> {
        let mut tags = Vec::<Tag>::new();
        match decl {
            Decl::ConTypeDecl(con_ty, (vid, ins, r)) => {
                let (c, t) = con_ty.get_con_elem().unwrap();
                let i_tag = Tag::Bound((c.clone(), t.clone()), Box::new(ins.clone().into_iter().collect::<Vec<String>>()));
                tags.push(i_tag);
                match self.analyse_ref(r.deref(), vid) {
                    Ok(prop_tags) => {
                        let mut prop_tags_mut = prop_tags.clone();
                        tags.append(&mut prop_tags_mut);
                        let con_tag = Tag::Con(t, String::new(), Box::new(tags));
                        self.ctx.put(c, con_tag);
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            },
            _ => Err("Not a valid container type declaration".to_string())
        }
    }

    fn analyse_ref<'a>(&self, r: &'a Refinement, vid: &'a Box<String>) -> Result<Vec<Tag>, AnalyserError> {
        match r {
            Refinement::Prop(term) => {
                match term.deref() {
                    Term::AppTerm(term1, term2) => {
                        match self.retrive_ref_term(term1) {
                            Ok(t) => {
                                let tags = vec![t.clone()];
                                Ok(tags)
                            },
                            Err(e) => Err(e)
                        }
                    },
                    _ => Err("Not a valid term for refining the type Con<T>".to_string())
                }
            },
            Refinement::AndProps(r1, r2) => {
                match self.analyse_ref(r1, vid) {
                    Ok(tags1) => {
                        match self.analyse_ref(r2, vid) {
                            Ok(tags2) => {
                                Ok([tags1, tags2].concat())
                            },
                            Err(e) => Err(e)
                        }
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }

    fn retrive_ref_term(&self, term: &Term) -> Result<&Tag, AnalyserError> {
        match term {
            Term::VarTerm(id) => {
                match self.ctx.get_id(id.to_string()) {
                    Some(t) => {
                        match t {
                            Tag::Prop(_) => Ok(t),
                            _ => Err(id.to_string() + " does not have a valid property")
                        }
                    },
                    _ => Err("Undefined variable: ".to_string() + id)
                }
            },
            _ => Err("Should be a varible term".to_string())
        }
    }

    pub fn analyse_term(&self, term: &mut Term, is_outter_app: bool, is_quantifier: bool, cdr_added: &mut Vec<String>, symbolics: &mut Vec<String>) -> String {
        match term {
            Term::LitTerm(lit) => {
                if (lit.to_string() == "true".to_string()) {
                    "#t".to_string()
                } else {
                    "#f".to_string()
                }
            },
            Term::VarTerm(id) => {
                id.to_string()
            },
            Term::LambdaTerm((id, _), t) => {
                if (is_quantifier) {
                    symbolics.push(id.to_string());
                    "(list ".to_string() + id + ") " + &self.analyse_term(t, true, false, cdr_added, symbolics) 
                } else {
                    "(lambda (".to_string() + id + ") " + &self.analyse_term(t, true, false, cdr_added, symbolics) + ")" 
                }
                               
            },
            Term::AppTerm(t1, t2) => {
                // Temporary solution of cdr required to adjust model ops
                if ((*t1.clone()).require_cdr() && !cdr_added.contains(&t1.to_string())) {
                    cdr_added.push(t1.to_string());
                    *term = Term::AppTerm(Box::new(Term::VarTerm(Box::new("cdr".to_string()))), Box::new(term.clone()));
                    self.analyse_term(term, is_outter_app, is_quantifier, cdr_added, symbolics)
                } else {
                    let mut result = String::new();
                    match ((*t1.clone()).is_quantifier(), *t2.clone()) {
                        (_, Term::AppTerm(_, _)) => {
                            if (is_outter_app) {
                                "(".to_string() + &self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, true, false, cdr_added, symbolics) + ")"
                            } else {
                                self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, true, false, cdr_added, symbolics)
                            }
                        },
                        (false, _) => {
                            if (is_outter_app) {
                                "(".to_string() + &self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, false, false, cdr_added, symbolics) + ")"
                            } else {
                                self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, false, false, cdr_added, symbolics)
                            }
                        },
                        (true, _) => {
                            if (is_outter_app) {
                                "(".to_string() + &self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, false, true, cdr_added, symbolics) + ")"
                            } else {
                                self.analyse_term(t1, false, false, cdr_added, symbolics) + " " + &self.analyse_term(t2, false, true, cdr_added, symbolics)
                            }
                        }
                    }
                }
            }
        }
    }

    fn write_prop_spec_file(&self, filename : String, contents: String, symbolics: String) -> Result<(), Error> {
        let mut output = fs::File::create(GENPATH.to_owned() + &filename)?;
        write!(output, "{}", LANGDECL.to_string())?;
        write!(output, "{}", REQUIRE.to_string())?;
        write!(output, "{}", EXTRAREQUIRE.to_string())?;
        write!(output, "{}", LISTMODEL.to_string())?;
        write!(output, "{}", contents)?;
        write!(output, "{}", symbolics)?;
        Ok(())
    }
}
