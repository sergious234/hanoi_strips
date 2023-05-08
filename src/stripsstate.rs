use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
};

use hashbrown::HashSet;

use crate::{
    accion::{Apilar, Meta},
    stackeable::Stackeable,
};
use std::ops::Deref;

type RecType = HashSet<Meta>;

const SOLUCION_SIZE: [usize; 14] = [
    1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383,
];

const STACK_SIZE: [usize; 14] = [2, 8, 14, 19, 24, 29, 34, 39, 44, 49, 54, 59, 64, 69];

const RECURSOS_SIZE: [usize; 14] = [
    7, 14, 28, 56, 56, 56, 112, 112, 112, 112, 224, 224, 224, 224,
];

#[derive(Clone)]
pub struct StripsState {
    pub stack_objetivos: VecDeque<Stackeable>,
    pub recursos: RecType,
    pub solucion: Vec<Apilar>,
    //pub pre_req_buffer: Option<[Rc<Stackeable>; 4]>,
    //pub buffer_len: usize,
}

impl PartialEq for StripsState {
    fn eq(&self, other: &Self) -> bool {
        if self.stack_objetivos.len() != other.stack_objetivos.len() {
            return false;
        }

        /*
         * Muy importante esta linea, añade una mejora de ~200ms para
         * 11 discos en mi ordenador.
         *
         */
        assert_eq!(self.stack_objetivos.len(), other.stack_objetivos.len());
        for i in 0..self.stack_objetivos.len() {
            if self.stack_objetivos.get(i) != other.stack_objetivos.get(i) {
                return false;
            }
        }

        /*
         * Comprobar si el len() de ambos sets empeora el rendimiento
         * porque el metodo is_subset parece que ya comprueba eso.
         */
        return self.recursos.is_subset(&other.recursos)
        /*
        if !self.recursos.is_subset(&other.recursos) {
            return false;
        }

        return true;
        */

    }
}

impl Eq for StripsState {}

impl StripsState {
    pub fn new(ea: Vec<Meta>, objetivos: Vec<Stackeable>, n_discos: usize) -> StripsState {
        //  HardCodear esto da una mejora de unos ~100ms
        //  en talla 11

        let stack_size = STACK_SIZE
            .get(n_discos - 1)
            .cloned()
            .unwrap_or(*STACK_SIZE.last().unwrap() * 2);

        let recursos_size = RECURSOS_SIZE
            .get(n_discos - 1)
            .cloned()
            .unwrap_or(*RECURSOS_SIZE.last().unwrap() * 2);

        let solucion_size = SOLUCION_SIZE
            .get(n_discos - 1)
            .cloned()
            .unwrap_or(*SOLUCION_SIZE.last().unwrap() * 2);

        let mut so = VecDeque::with_capacity(stack_size);
        so.append(&mut objetivos.into());

        let mut rec = RecType::with_capacity(recursos_size);
        for r in ea.into_iter() {
            rec.insert(r);
        }

        StripsState {
            stack_objetivos: so,
            solucion: Vec::with_capacity(solucion_size),
            recursos: rec,
        }
    }

    #[inline]
    pub fn cumple_meta(&self, meta: Meta) -> bool {
        self.recursos.contains(&meta)
    }

    #[inline]
    pub fn cumple_conjuncion(&self, conjuncion: &[Meta]) -> bool {
        conjuncion.iter().all(|meta| self.recursos.contains(meta))
    }

    #[inline]
    pub fn cumple_meta_bucle(&self, meta: Meta) -> bool {
        let item = Stackeable::Objetivo(meta);

        for i in 0..self.stack_objetivos.len() - 1 {
            if self.stack_objetivos.get(i).unwrap().deref() == &item {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn add_metas(&mut self, metas: [Meta; 2]) {
        metas
            .into_iter()
            .for_each(|m| self.stack_objetivos.push_back(Stackeable::Objetivo(m)));
    }

    /*
    pub fn buffer_empty(&self) -> bool {
        self.buffer_len == 0
    }
    */

    #[inline]
    pub fn copy(&self) -> Self {
        let o_stack = self.stack_objetivos.clone();
        let o_rec = self.recursos.clone();
        let o_solucion = self.solucion.clone();

        StripsState {
            solucion: o_solucion,
            recursos: o_rec,
            stack_objetivos: o_stack,
        }
    }
}

impl Hash for StripsState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for r in &self.recursos {
            r.hash(state)
        }

        let mut i: u64 = 33;
        for objetivo in &self.stack_objetivos {
            objetivo.hash(state);
            i += 1;
        }
        i = i + 0x9e3779b9 + (i << 6) + (i >> 2);
        i.hash(state);
    }
}
