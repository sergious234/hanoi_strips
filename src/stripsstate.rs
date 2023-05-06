use std::{
    collections::{VecDeque, BTreeSet},
    hash::{Hash, Hasher},
};

use std::rc::Rc;

use crate::{
    accion::{Apilar, Meta},
    stackeable::Stackeable,
};
use std::ops::Deref;


type RecType = BTreeSet<Meta>;


#[derive(Clone)]
pub struct StripsState {
    pub stack_objetivos: VecDeque<Stackeable>,
    pub recursos: RecType, //BTreeSet<Rc<Meta>>,
    pub solucion: Vec<Apilar>, 
    //pub pre_req_buffer: Option<[Rc<Stackeable>; 4]>,
    //pub buffer_len: usize,
}

impl PartialEq for StripsState {
    fn eq(&self, other: &Self) -> bool {
        if self.stack_objetivos.len() != other.stack_objetivos.len() {
            return false;
        }
        assert_eq!(self.stack_objetivos.len(), other.stack_objetivos.len());
        for i in 0..self.stack_objetivos.len() {
            if self.stack_objetivos.get(i) != other.stack_objetivos.get(i) {
                return false;
            }
        }

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
        let n_obj = objetivos.into_iter()
            .map(|o| o)
            .collect();

        
        let n_recursos: Vec<Rc<Meta>> = ea.into_iter()
            .map(|r| Rc::new(r))
            .collect();
        */
        StripsState {
            stack_objetivos: objetivos.into(),
            solucion: Vec::with_capacity(325),
            recursos: RecType::from_iter(ea),
            //pre_req_buffer: None,
//            buffer_len: 0
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
    pub fn add_metas(&mut self, metas: Vec<&Meta>) {
        metas.into_iter().for_each(|m| {
            self.stack_objetivos
                .push_back(Stackeable::Objetivo(*m).into())
        });
    }

    /*
    pub fn buffer_empty(&self) -> bool {
        self.buffer_len == 0
    }
    */

    pub fn copy(&self) -> Self {
        let o_stack = self.stack_objetivos.clone(); //VecDeque::with_capacity(self.stack_objetivos.len());
        let o_rec = self.recursos.clone();
        let o_solucion = self.solucion.clone(); //Vec::with_capacity(self.solucion.len());

        StripsState{
            solucion: o_solucion,
            recursos: o_rec,
            stack_objetivos: o_stack,
            //pre_req_buffer: None,
            //buffer_len: 0,
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
