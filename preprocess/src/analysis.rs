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
const GENPATH: &str = "./racket_specs/gen_prop_spec/";

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
        let interface_decls: Vec<&Decl> =
            concat_specs.iter()
            .filter(| decl | decl.is_interface_decl())
            .collect();
        match self.analyse_prop_decls(prop_decls) {
            Ok(_) => match self.analyse_interface_decls(interface_decls) {
                Ok(_) => self.analyse_contype_decls(contype_decls),
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
            Decl::PropertyDecl(id, term) => {
                let code =  "(define ".to_string() + id + " " + &self.analyse_term(term) + ")\n" + "(provide " + id + ")";
                let filename = id.to_string() + ".rkt";
                self.write_prop_spec_file(filename.clone(), code);
                let prop_tag = Tag::Prop(Box::new(id.to_string()));
                self.ctx.put(id.to_string(), prop_tag);
                self.prop_specs.insert(id.to_string(), filename);
                Ok(())
            },
            _ => Err("Not a valid property declaration".to_string())
        }
    }

    pub fn analyse_interface_decls(&mut self, decls: Vec<&Decl>) -> Result<(), AnalyserError> {
        let mut result = Ok(());
        for decl in decls.into_iter() {
            match self.analyse_interface_decl(decl) {
                Ok(_) => continue,
                Err(e) => result = Err(e)
            }
        }
        result
    }

    pub fn analyse_interface_decl(&mut self, decl: &Decl) -> Result<(), AnalyserError> {
        match decl {
            Decl::InterfaceDecl(id, interfaces) => {
                let interface_tag = Tag::Interface(Box::new(interfaces.to_vec()));
                self.ctx.put(id.to_string(), interface_tag);
                Ok(())
            },
            _ => Err("Not a valid interface declaration".to_string())
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
            Decl::ConTypeDecl(con_ty, (vid, inid, r)) => {
                match self.ctx.get_id(inid.to_string()) {
                    Some(tag) => {
                        tags.push(tag.clone());
                        match self.analyse_ref(r.deref(), vid) {
                            Ok(prop_tags) => {
                                let mut prop_tags_mut = prop_tags.clone();
                                tags.append(&mut prop_tags_mut);
                                let (c, t) = con_ty.get_con_elem().unwrap();
                                let con_tag = Tag::Con(t, Box::new(tags));
                                self.ctx.put(con_ty.to_string(), con_tag);
                                Ok(())
                            },
                            Err(e) => Err(e)
                        }
                    },
                    None => Err("Interface ".to_string() + inid + " is not defined.")
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
        let mut output = fs::File::create(GENPATH.to_owned() + &filename)?;
        write!(output, "{}", LANGDECL.to_string())?;
        write!(output, "{}", REQUIRE.to_string())?;
        write!(output, "{}", contents)?;
        Ok(())
    }
}
