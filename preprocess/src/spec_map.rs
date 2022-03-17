use std::collections::HashMap;
use std::collections::hash_map::Iter;

type StructName = String;
type LibSpecDir = String;
type BoundName = String;
type BoundProvide = String;
type MatchSetupDir = String;
pub type Bounds = HashMap<BoundName, BoundProvide>;

type PropertyName = String;
type PropSpecDir = String;

pub type LibSpecs = HashMap<StructName, (LibSpecDir, Bounds)>;
pub type PropSpecs = HashMap<PropertyName, PropSpecDir>;
pub type MatchSetup = HashMap<BoundName, MatchSetupDir>;