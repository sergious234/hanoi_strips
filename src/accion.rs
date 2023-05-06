use crate::{stackeable::Stackeable, stripsstate::StripsState};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, Eq, Copy, PartialOrd, Ord)]
pub enum Meta {
    Sobre(i32, i32),
    Menor(i32, i32),
    Despejado(i32),
}

#[derive(Clone, Ord, PartialOrd)]
pub struct Apilar {
    x: i32,
    y: i32,
    z: i32,
    pub lista_adicion: [Rc<Meta>; 2], //Vec<Rc<Meta>>,
    pub lista_supresion: [Rc<Meta>; 2], // Vec<Rc<Meta>>,
    pub precondiciones: [Rc<Meta>; 4], // Vec<Rc<Meta>>,
}

impl PartialEq for Apilar {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Apilar {}

impl Hash for Apilar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.write_i32(self.z);
        state.finish();
    }
}

impl Apilar {
    pub fn to_string(&self) -> String {
        format!("Apilar {}|{:2}|{:2}", self.x, self.y, self.z)
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let lista_adicion: [Rc<Meta>; 2] = [
            Meta::Despejado(y).into(),
            Meta::Sobre(x,z).into()
        ];

        let precondiciones: [Rc<Meta>; 4] = [
            Meta::Menor(x, z).into(),
            Meta::Sobre(x, y).into(),
            Meta::Despejado(x).into(),
            Meta::Despejado(z).into() 
        ];

        let lista_supresion: [Rc<Meta>; 2] = [
            Meta::Despejado(z).into(),
            Meta::Sobre(x,y).into()
        ];

        Apilar {
            x,
            y,
            z,
            lista_adicion,
            lista_supresion,
            precondiciones,
        }
    }

    pub fn es_ejecutable(&self, estado_actual: &StripsState) -> bool {
        self.precondiciones
            .iter()
            .all(|item| estado_actual.recursos.contains(item))
    }

    pub fn aplica_accion(&self, estado_actual: &StripsState) -> StripsState {
        let mut estado_copia = estado_actual.copy();

        estado_copia.stack_objetivos.pop_back();
        self.lista_supresion.iter().for_each(|item| {
            estado_copia.recursos.remove(item);
        });
        self.lista_adicion.iter().for_each(|item| {
            estado_copia.recursos.insert(item.clone());
        });
        estado_copia.solucion.push(self.clone());
        estado_copia
    }

    pub fn add_prerequisitos(&self, estado_actual: &StripsState) -> StripsState {
        let mut copia: StripsState = estado_actual.copy();

        //assert_eq!(self.precondiciones.len(), 4);
        self.precondiciones.iter().take(2).for_each(|pre| {
            copia
                .stack_objetivos
                .push_back(Stackeable::Objetivo(pre.deref().clone()).into())
        });

        let mut conj = Vec::with_capacity(2);
        for i in 2..4 {
            conj.push(self.precondiciones[i].deref().clone());
        }
        // let conj = self.precondiciones.iter()
        // .skip(2)
        // .map(|f| f.clone())
        // .collect::<Vec<Meta>>();

        // let buffer: [Rc<Stackeable>; 4] = [
        //     Stackeable::Objetivo(self.precondiciones.get(0).unwrap().deref().clone().into()).into(),
        //     Stackeable::Objetivo(self.precondiciones.get(1).unwrap().deref().clone().into()).into(),
        //     Stackeable::Conjuncion(conj.clone()).into(),
        //     Stackeable::Objetivo(self.precondiciones.get(1).unwrap().deref().clone().into()).into(),
        // ];
        //
        // copia.pre_req_buffer = Some(buffer);
        // copia.buffer_len = 3;

        copia
            .stack_objetivos
            .push_back(Rc::new(Stackeable::Conjuncion(conj)));

        copia
    }

    pub fn genera_posibilidades(&self, meta: &Meta, estado_actual: &StripsState) -> Vec<Apilar> {
        let mut posibilidades: Vec<Apilar> = Vec::new();

        match meta {
            Meta::Sobre(x, y) => {
                for meta in estado_actual.recursos.iter() {
                    if let Meta::Sobre(x2, y2) = meta.deref() {
                        if x == x2 && x != y2 && (x <= y || *y < 0) {
                            posibilidades.push(Apilar::new(*x, *y2, *y))
                        }
                    }
                }
            }

            Meta::Despejado(x) => {
                let mut coso_que_mover = 0;

                estado_actual.recursos.iter().for_each(|item| {
                    if let Meta::Sobre(x2, y2) = item.deref() {
                        if x == y2 {
                            coso_que_mover = *x2
                        }
                    }
                });

                //let mut despejados: Vec<&i32> = Vec::new();
                let despejados: Vec<&i32> = estado_actual
                    .recursos
                    .iter()
                    .filter_map(|item| {
                        if let Meta::Despejado(x2) = item.deref() {
                            if *x2 < 0 || *x2 > coso_que_mover {
                                return Some(x2);
                            }
                        }
                        None
                    })
                    .collect();
                // estado_actual.recursos.iter().for_each(|item| {
                //     if let Meta::Despejado(x2) = item {
                //         if *x2 < 0 || *x2 > coso_que_mover {
                //             despejados.push(x2)
                //         }
                //     }
                // });

                for despejado in despejados {
                    posibilidades.push(Apilar::new(coso_que_mover, *x, *despejado))
                }
            }

            Meta::Menor(_, _) => {}
        }
        //posibilidades.sort();
        posibilidades
    }
}
