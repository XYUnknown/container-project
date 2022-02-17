pub trait ContainerConstructor {
    type Impl: ?Sized;
    type Interface: ?Sized;
    fn new() -> Box<Self::Interface>;
}