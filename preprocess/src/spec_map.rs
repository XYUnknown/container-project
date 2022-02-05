use std::collections::HashMap;
use std::collections::hash_map::Iter;

type StructName = String;
type LibSpecDir = String;

type PropertyName = String;
type PropSpecDir = String;

pub type LibSpecs = HashMap<StructName, LibSpecDir>;
pub type PropSpecs = HashMap<PropertyName, PropSpecDir>;