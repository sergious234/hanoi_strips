use crate::accion::{Apilar, Meta};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Stackeable {
    Accion(Apilar),
    Objetivo(Meta),
    Conjuncion([Meta; 2]),
}

impl Stackeable {
    pub fn is_accion(&self) -> bool {
        match self {
            Self::Accion(_) => true,
            _ => false,
        }
    }
}
