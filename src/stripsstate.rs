use std::{
    collections::{BTreeSet, VecDeque},
    hash::{Hash, Hasher},
};


use hashbrown::HashSet;

use crate::{
    accion::{Apilar, Meta},
    stackeable::Stackeable,
};
use std::ops::Deref;

type RecType = HashSet<Meta>;

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
        if !self.recursos.is_subset(&other.recursos) {
            return false;
        }

        return true;
    }
}

impl Eq for StripsState {}

// impl Clone for StripsState {
//     fn clone(&self) -> Self {
//         let other_stack = self.stack_objetivos.clone();
//     }
// }

impl StripsState {
    pub fn new(ea: Vec<Meta>, objetivos: Vec<Stackeable>) -> StripsState {
        /*
         * HardCodear esto da una mejora de unos ~100ms
         * en talla 11
         *
         *
         */
        let mut so = VecDeque::with_capacity(89);
        so.append(&mut objetivos.into());

        let mut rec = RecType::with_capacity(224);
        for r in ea.into_iter(){
            rec.insert(r);
        }

        StripsState {
            stack_objetivos: so,
            solucion: Vec::with_capacity(325),
            recursos: rec,
        }
    }

    #[inline]
    pub fn cumple_meta(&self, meta: &Meta) -> bool {
        self.recursos.contains(meta)
    }

    #[inline]
    pub fn cumple_conjuncion(&self, conjuncion: &[Meta]) -> bool {
        conjuncion.iter().all(|meta| self.recursos.contains(meta))
    }

    #[inline]
    pub fn cumple_meta_bucle(&self, meta: &Meta) -> bool {
        let item = Stackeable::Objetivo(*meta);

        for i in 0..self.stack_objetivos.len() - 1 {
            if self.stack_objetivos.get(i).unwrap().deref() == &item {
                return true;
            }
        }

        return false;
    }

    #[inline]
    pub fn add_metas(&mut self, metas: [Meta; 2]) {
        metas.into_iter().for_each(|m| {
            self.stack_objetivos
                .push_back(Stackeable::Objetivo(m).into())
        });
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
            i = i + 0x9e3779b9 + (i << 6) + (i >> 2);
        }
        i.hash(state);
    }
}
