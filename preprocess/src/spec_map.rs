use std::collections::HashMap;
use std::collections::hash_map::Iter;

type StructName = String;
type LibSpecDir = String;
type BoundName = String;
type BoundProvide = String;
type MatchSetupDir = String;
pub type Bounds = HashMap<BoundName, BoundProvide>;
pub type ProvidedOps = (Vec<String>, Vec<String>);

type PropertyName = String;
type PropSpecDir = String;
type PropSymbolics = Vec<String>;

pub type LibSpecs = HashMap<StructName, (LibSpecDir, Bounds, ProvidedOps)>;
pub type PropSpecs = HashMap<PropertyName, (PropSpecDir, PropSymbolics)>;
pub type MatchSetup = HashMap<BoundName, MatchSetupDir>;