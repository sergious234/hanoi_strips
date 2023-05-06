use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};
use std::ops::Deref;
use std::rc::Rc;

use crate::{
    accion::{Apilar, Meta},
    stackeable::Stackeable,
    stripsstate::StripsState,
};

enum EstadoMeta {
    CumpleMeta,
    Bucle,
    Igual,
    NuevosEstados,
    Acc,
}

#[allow(dead_code)]
pub struct Strips {
    estados: VecDeque<StripsState>,
    visitados: HashSet<StripsState>,
    acciones_disponibles: Apilar,
    objetivo_meta: Vec<Meta>,
}

impl Strips {
    pub fn new(estado_inicial: StripsState, acciones: Apilar, meta: Vec<Meta>) -> Strips {
        let mut s = Strips {
            estados: VecDeque::new(),
            visitados: HashSet::with_capacity(826548),
            acciones_disponibles: acciones,
            objetivo_meta: meta,
        };
        s.estados.push_back(estado_inicial);
        return s;
    }

    pub fn resolver(mut self) {
        let mut its = 0;
        // let estado_actual;
        let start = Instant::now();
        while !self.estados.is_empty() {
            let estado_actual = self.estados.pop_back().expect("No quedan estados WTF");
            if estado_actual.stack_objetivos.is_empty() {
                let end = Instant::now();
                println!("{} Solucion: ", self.visitados.len());
                estado_actual
                    .solucion
                    .iter()
                    .for_each(|e| println!("{:?}", e.to_string()));
                println!("{}ms", end.duration_since(start).as_millis());
                break;
            }

            its += 1;
            if self.visitados.contains(&estado_actual) {
                continue;
            }

            self.prueba_estado(&estado_actual);
            self.visitados.insert(estado_actual);
        }
        println!("Terminamos ! Its: {} ", self.visitados.len());
    }

    pub fn prueba_estado(&mut self, estado_actual: &StripsState) {
        let elemento = estado_actual
            .stack_objetivos
            .back()
            .expect("No quedan objetivos WTF");

        let estado = match elemento.deref() {
            Stackeable::Accion(acc) => {
                let copia = if acc.es_ejecutable(estado_actual) {
                    acc.aplica_accion(estado_actual)
                } else {
                    acc.add_prerequisitos(estado_actual)
                };

                self.estados.push_back(copia);
                EstadoMeta::Acc
            }
            Stackeable::Objetivo(ref meta_actual) => self.meta_simple(estado_actual, meta_actual),
            Stackeable::Conjuncion(ref con) => self.meta_compuesta(estado_actual, con),
        };

        match estado {
            EstadoMeta::CumpleMeta => {
                let mut copia = estado_actual.clone();
                copia.stack_objetivos.pop_back();
                self.estados.push_back(copia);
            }
            _ => {}
        }
    }

    fn meta_simple(&mut self, estado_actual: &StripsState, meta_actual: &Meta) -> EstadoMeta {
        if estado_actual.cumple_meta(meta_actual) {
            return EstadoMeta::CumpleMeta;
        }
        if Strips::hay_bucle(estado_actual, meta_actual) {
            return EstadoMeta::Bucle;
        }

        let mut siguientes_estados = VecDeque::new();

        let posibilidades = self
            .acciones_disponibles
            .genera_posibilidades(meta_actual, estado_actual);

        for pos in posibilidades {
            let mut copia = estado_actual.copy();
            copia.stack_objetivos.push_back(Rc::new(Stackeable::Accion(pos)));
            siguientes_estados.push_back(copia);
        }

        if siguientes_estados.is_empty() {
            return EstadoMeta::Igual;
        } else {
            self.estados.append(&mut siguientes_estados);
            return EstadoMeta::NuevosEstados;
        }
    }

    fn meta_compuesta(&mut self, estado_actual: &StripsState, conj: &[Meta]) -> EstadoMeta {
        if estado_actual.cumple_conjuncion(&conj) {
            return EstadoMeta::CumpleMeta;
        }

        let len: usize = conj.len();
        let permutaciones: Vec<Vec<&Meta>> =
            conj.iter().permutations(len).unique().collect();

        permutaciones.into_iter().for_each(|metas| {
            let mut copia = estado_actual.clone();
            copia.stack_objetivos.pop_back();
            copia.add_metas(metas);
            self.estados.push_back(copia);
        });

        EstadoMeta::NuevosEstados
    }

    fn hay_bucle(estado_actual: &StripsState, meta_actual: &Meta) -> bool {
        estado_actual.cumple_meta_bucle(meta_actual)
    }
}
