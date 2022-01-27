use crate::description::{Tag, InforMap};
use crate::parser::{Prog, Block, Spec, Decl, Term, Refinement, Id, spec};
use std::ops::Deref;
use std::env;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

type AnalyserError = String;
const LANGDECL: &str = "#lang rosette\n";

pub struct Analyser {
    ctx: InforMap,
}

impl Analyser {
    pub fn new() -> Analyser {
        Analyser {
            ctx: InforMap::new(),
        }
    }

    pub fn get_ctx(&self) -> &InforMap {
        &self.ctx
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
            Ok(_) => self.analyse_contype_decls(contype_decls),
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
            Decl::PropertyDecl(id, term) => {
                // TODO: actually analyse term
                let code =  "(define ".to_string() + id + " " + &self.analyse_term(term) + ")";
                let filename = id.to_string() + ".rkt";
                self.write_prop_spec_file(filename, code);
                let prop_tag = Tag::Prop(Box::new(id.to_string()));
                self.ctx.put(id.to_string(), prop_tag);
                Ok(())
            },
            _ => Err("Not a valid property declaration".to_string())
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
        match decl {
            Decl::ConTypeDecl(id, (vid, ty, r)) => {
                match self.analyse_ref(r.deref(), vid) {
                    Ok(tags) => {
                        let con_tag = Tag::Con(Box::new(tags.to_vec()));
                        self.ctx.put(id.to_string(), con_tag);
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

    pub fn analyse_term(&self, term: &Term) -> String {
        match term {
            Term::VarTerm(id) => {
                id.to_string()
            },
            Term::LambdaTerm(id, t) => {
                "(lambda (".to_string() + id + ") (" + &self.analyse_term(t) + "))"
            },
            Term::AppTerm(t1, t2) => {
                self.analyse_term(t1) + " " + &self.analyse_term(t2)
            }
        }
    }

    fn write_prop_spec_file(&self, filename : String, contents: String) -> Result<(), Error> {
        let path = "./gen_prop_spec/";
        let mut output = fs::File::create(path.to_owned() + &filename)?;
        write!(output, "{}", LANGDECL.to_string())?;
        write!(output, "{}", contents)?;
        Ok(())
    }
}
