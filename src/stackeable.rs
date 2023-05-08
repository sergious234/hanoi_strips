use crate::accion::{Apilar, Meta};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stackeable {
    Accion(Apilar),
    Objetivo(Meta),
    Conjuncion([i8; 2]),
}

impl Stackeable {
    pub fn is_accion(&self) -> bool {
        matches!(self, Self::Accion(_))
            /*
        match self {
            Self::Accion(_) => true,
            _ => false,
        }
        */
    }
}
