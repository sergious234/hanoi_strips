use crate::{stackeable::Stackeable, stripsstate::StripsState};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Clone, Hash, PartialEq, Eq, Copy, PartialOrd, Ord)]
pub enum Meta {
    Sobre(i8, i8),
    Menor(i8, i8),
    Despejado(i8),
}

pub struct Posibilidades {
    pub acciones: [Apilar; 2],
    pub elements: usize,
}

impl Posibilidades {
    pub fn new() -> Posibilidades {
        Posibilidades {
            acciones: [Apilar::default(), Apilar::default()],
            elements: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, acc: Apilar) {
        self.acciones[self.elements] = acc;
        self.elements += 1;
    }
}

impl Meta {
    #[inline]
    pub fn get_first(&self) -> i8 {
        match *self {
            Meta::Sobre(x, _) => x,
            Meta::Menor(x, _) => x,
            Meta::Despejado(x) => x,
        }
    }

    pub fn reversed(&self) -> Meta {
        match *self {
            Meta::Sobre(x, y) => Meta::Sobre(y, x),
            Meta::Menor(x, y) => Meta::Menor(y, x),
            Meta::Despejado(_) => *self,
        }
    }
}

type MetaS = Meta;

#[derive(Default, Clone, Ord, Copy, PartialOrd)]
pub struct Apilar {
    pub x: i8,
    y: i8,
    z: i8,
}

impl PartialEq for Apilar {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Apilar {}

impl Hash for Apilar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(self.x);
        state.write_i8(self.y);
        state.write_i8(self.z);
        state.finish();
    }
}

impl Apilar {
    pub fn to_string(&self) -> String {
        format!("Apilar {}|{:2}|{:2}", self.x, self.y, self.z)
    }

    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Apilar { x, y, z }
    }

    #[inline]
    fn lista_supresion(&self) -> [MetaS; 2] {
        [Meta::Despejado(self.z), Meta::Sobre(self.x, self.y)]
    }

    #[inline]
    fn lista_adicion(&self) -> [MetaS; 2] {
        [Meta::Despejado(self.y), Meta::Sobre(self.x, self.z)]
    }

    pub fn es_ejecutable(&self, estado_actual: &StripsState) -> bool {
        let precondiciones: [MetaS; 4] = [
            Meta::Menor(self.x, self.z),
            Meta::Sobre(self.x, self.y),
            Meta::Despejado(self.x),
            Meta::Despejado(self.z),
        ];

        precondiciones
            .iter()
            .all(|item| estado_actual.recursos.contains(item))
    }

    pub fn aplica_accion(&self, estado_actual: &StripsState) -> StripsState {
        let mut estado_copia = estado_actual.copy();

        estado_copia.stack_objetivos.pop_back();
        self.lista_supresion().iter().for_each(|item| {
            estado_copia.recursos.remove(item);
        });
        self.lista_adicion().iter().for_each(|item| {
            estado_copia.recursos.insert(*item);
        });
        estado_copia.solucion.push(*self);
        estado_copia
    }

    pub fn add_prerequisitos(&self, estado_actual: &StripsState) -> StripsState {
        let mut copia: StripsState = estado_actual.copy();

        let precondiciones: [MetaS; 4] = [
            Meta::Menor(self.x, self.z),
            Meta::Sobre(self.x, self.y),
            Meta::Despejado(self.x),
            Meta::Despejado(self.z),
        ];

        //assert_eq!(self.precondiciones.len(), 4);
        precondiciones.iter().take(2).for_each(|pre| {
            copia
                .stack_objetivos
                .push_back(Stackeable::Objetivo(*pre.deref()))
        });

        let conj = [precondiciones[2].get_first(), precondiciones[3].get_first()]; //Vec::with_capacity(2);

        copia
            .stack_objetivos
            .push_back(Stackeable::Conjuncion(conj));

        copia
    }

    pub fn genera_posibilidades(&self, meta: Meta, estado_actual: &StripsState) -> Posibilidades {
        //let mut posibilidades: Vec<Apilar> = Vec::new();
        let mut posibilidades = Posibilidades::new();
        let last_mov = if !estado_actual.solucion.is_empty() {
            estado_actual.solucion.last().unwrap().x
        } else {
            0
        };

        match meta {
            Meta::Sobre(x, y) => {
                for meta in estado_actual.recursos.iter() {
                    if let Meta::Sobre(x2, y2) = *meta {
                        if x == x2 && x != y2 && (x <= y || y < 0) && x != last_mov {
                            posibilidades.push(Apilar::new(x, y2, y))
                        }
                    }
                }
            }

            Meta::Despejado(x) => {
                let mut coso_que_mover = 0;

                for recurso in &estado_actual.recursos {
                    if let Meta::Sobre(x2, y2) = *recurso {
                        if x == y2 {
                            coso_que_mover = x2;
                            break;
                        }
                    }
                }

                if coso_que_mover != last_mov {
                    estado_actual.recursos.iter().for_each(|item| {
                        if let Meta::Despejado(donde) = *item {
                            if donde < 0
                                || (donde > coso_que_mover && donde % 2 != coso_que_mover % 2)
                            {
                                posibilidades.push(Apilar::new(coso_que_mover, x, donde));
                            }
                        }
                    });
                }
            }

            Meta::Menor(_, _) => {}
        }
        posibilidades
    }
}
