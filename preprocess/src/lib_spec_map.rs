use std::collections::HashMap;
use std::collections::hash_map::Iter;

type StructName = String;
type LibSpecDir = String;

pub type LibSpecs = HashMap<StructName, LibSpecDir>;