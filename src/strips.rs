use itertools::Itertools;

use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use hashbrown::HashSet;

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

const VISITADOS_SIZE: [usize; 14] = [
    3, 14, 37, 247, 437, 1065, 2417, 5386, 11876, 25033, 55892, 117509, 253839, 552215,
];

const MULTI_VISITADOS_SIZE: [usize; 12] =
    [2, 3, 6, 42, 110, 348, 968, 2957, 8657, 24757, 73336, 216709];

#[allow(dead_code)]
pub struct Strips {
    estados: VecDeque<StripsState>,
    visitados: HashSet<StripsState>,
    acciones_disponibles: Apilar,
    objetivo_meta: Vec<Meta>,
}

impl Strips {
    pub fn new(
        estado_inicial: StripsState,
        acciones: Apilar,
        meta: Vec<Meta>,
        n_discos: i8,
    ) -> Strips {
        let visitados_size: usize = VISITADOS_SIZE
            .get((n_discos - 1) as usize)
            .cloned()
            .unwrap_or(VISITADOS_SIZE.last().unwrap() * 2);

        /*
        let multi_visitados_size = MULTI_VISITADOS_SIZE[(n_discos - 1) as usize];
        let mut visitados_map = HashMap::with_capacity(10);
        for i in 0..10 {
            visitados_map.insert(i, HashSet::with_capacity(multi_visitados_size));
        }
        */

        let mut s = Strips {
            estados: VecDeque::with_capacity(2693),
            visitados: HashSet::with_capacity(visitados_size),
            acciones_disponibles: acciones,
            objetivo_meta: meta,
        };
        s.estados.push_back(estado_inicial);
        return s;
    }

    pub fn resolver(mut self) {
        let start = Instant::now();

        let max_stack = 0;
        let max_recursos = 0;
        let max_solucion = 0;

        while !self.estados.is_empty() {
            // assert!(!self.estados.is_empty());
            let estado_actual = self.estados.pop_back().expect("No quedan estados WTF");

            /*
            if estado_actual.stack_objetivos.len() > max_stack {
                max_stack = estado_actual.stack_objetivos.len();
            }

            if estado_actual.recursos.capacity() > max_recursos {
                max_recursos = estado_actual.recursos.capacity();
            }

            if estado_actual.solucion.len() > max_solucion {
                max_solucion = estado_actual.solucion.len();
            }
            */

            if estado_actual.stack_objetivos.is_empty() {
                let end = Instant::now();

                println!("Solucion: {}", estado_actual.solucion.len());

                /*
                estado_actual
                    .solucion
                    .iter()
                    .for_each(|e| println!("{:?}", e.to_string()));
                    */

                println!("{}ms", end.duration_since(start).as_millis());
                break;
            }

            // its += 1;
            if self.visitados.contains(&estado_actual) {
                continue;
            }

            self.prueba_estado(&estado_actual);
            self.visitados.insert(estado_actual);
        }
    }

    #[inline]
    pub fn prueba_estado(&mut self, estado_actual: &StripsState) {
        let elemento = estado_actual
            .stack_objetivos
            .back()
            .expect("No quedan objetivos WTF");

        let estado = match *elemento {
            Stackeable::Accion(acc) => {
                let copia = if acc.es_ejecutable(estado_actual) {
                    acc.aplica_accion(estado_actual)
                } else {
                    acc.add_prerequisitos(estado_actual)
                };
                self.estados.push_back(copia);
                EstadoMeta::Acc
            }
            Stackeable::Objetivo(meta_actual) => self.meta_simple(estado_actual, meta_actual),
            Stackeable::Conjuncion(con) => self.meta_compuesta(estado_actual, con),
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

    #[inline]
    fn meta_simple(&mut self, estado_actual: &StripsState, meta_actual: Meta) -> EstadoMeta {
        if estado_actual.cumple_meta(&meta_actual) {
            return EstadoMeta::CumpleMeta;
        }
        if Strips::hay_bucle(estado_actual, &meta_actual) {
            return EstadoMeta::Bucle;
        }

        let mut siguientes_estados = VecDeque::new();

        let posibilidades = self
            .acciones_disponibles
            .genera_posibilidades(meta_actual, estado_actual);

        for pos in posibilidades {
            let mut copia = estado_actual.clone();
            copia
                .stack_objetivos
                .push_back(Stackeable::Accion(pos).into());
            siguientes_estados.push_back(copia);
        }

        if siguientes_estados.is_empty() {
            return EstadoMeta::Igual;
        } else {
            self.estados.append(&mut siguientes_estados);
            return EstadoMeta::NuevosEstados;
        }
    }

    #[inline]
    fn meta_compuesta(&mut self, estado_actual: &StripsState, conj: [i8; 2]) -> EstadoMeta {
        
        let conj: [Meta; 2] = [
            Meta::Despejado(conj[0]),
            Meta::Despejado(conj[1]),
        ];


        if estado_actual.cumple_conjuncion(&conj) {
            return EstadoMeta::CumpleMeta;
        }

        if conj.iter().any(|m| Strips::hay_bucle(estado_actual, m)) {
            return EstadoMeta::Bucle;
        }

        /*
         * Optimizacion para Hanoi:
         *  Como a esta metodo solo van a llegar 2 metas [A,B] se pueden
         *  crear las permutaciones a mano tal que:
         *      [A,B]
         *      [B,A]
         *
         *  Esto aumenta el rendimiento unos 100ms en mi maquina
         *  para DISCOS=9
         *
         * */
        let permutaciones: [[Meta; 2]; 2] = [[conj[0], conj[1]], [conj[1], conj[0]]];

        permutaciones.into_iter().for_each(|metas| {
            let mut copia = estado_actual.clone();
            copia.stack_objetivos.pop_back();
            copia.add_metas(metas);
            self.estados.push_back(copia);
        });

        EstadoMeta::NuevosEstados
    }

    #[inline]
    fn hay_bucle(estado_actual: &StripsState, meta_actual: &Meta) -> bool {
        estado_actual.cumple_meta_bucle(meta_actual)
    }
}
