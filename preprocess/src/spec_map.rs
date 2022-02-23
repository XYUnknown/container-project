use std::collections::HashMap;
use std::collections::hash_map::Iter;

type StructName = String;
type LibSpecDir = String;
type InterfaceName = String;
type InterfaceProvide = String;
type MatchSetupDir = String;
pub type Interfaces = HashMap<InterfaceName, InterfaceProvide>;

type PropertyName = String;
type PropSpecDir = String;

pub type LibSpecs = HashMap<StructName, (LibSpecDir, Interfaces)>;
pub type PropSpecs = HashMap<PropertyName, PropSpecDir>;
pub type MatchSetup = HashMap<InterfaceName, MatchSetupDir>;