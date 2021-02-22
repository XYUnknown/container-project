# Containers with Properties for Portable Performance

Designing a declarative representation of containers and their properties guiding code generation.

## directories
* ./agda_containers/
  * A generic container representation formalised in Agda
* ./rust_containers/
  * Different container types implemented in rust
  * WIP: Abstract a way for a user to get a container by calling something like: `Container c = genVec(Unique)`
