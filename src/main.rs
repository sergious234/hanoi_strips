
use strips::Strips;
use stripsstate::StripsState;

use crate::{
    accion::{Apilar, Meta},
    stackeable::Stackeable,
};

pub mod accion;
pub mod stackeable;
pub mod strips;
pub mod stripsstate;

fn main() {
    let mut estado_actual = Vec::new();
    const DISCOS: i8 = 8;

    estado_actual.push(Meta::Despejado(1));
    estado_actual.push(Meta::Despejado(-2));
    estado_actual.push(Meta::Despejado(-3));

    for disco_actual in 2..=DISCOS {
        for i in 1..disco_actual {
            estado_actual.push(Meta::Menor(i, disco_actual));
        }
    }

    for i in 1..=DISCOS {
        for varilla in -3..0 {
            estado_actual.push(Meta::Menor(i, varilla));
        }
    }

    for i in 1..DISCOS {
        estado_actual.push(Meta::Sobre(i, i + 1));
    }
    estado_actual.push(Meta::Sobre(DISCOS, -1));

    // Objetivos
    let mut objetivos = Vec::new();
    for i in 1..DISCOS {
        objetivos.push(Stackeable::Objetivo(Meta::Sobre(i, i + 1)));
    }
    objetivos.push(Stackeable::Objetivo(Meta::Sobre(DISCOS, -3)));

    // Estado final
    let mut meta_final = Vec::new();
    for i in 1..DISCOS {
        meta_final.push(Meta::Sobre(i, i + 1));
    }
    meta_final.push(Meta::Sobre(DISCOS, -3));


    println!("{}", u64::MAX % 128);
    let _visitados: [u128; 127] = [0; 127];

    let estado_inicial = StripsState::new(estado_actual, objetivos);
    let strips_solver = Strips::new(estado_inicial, Apilar::new(0, 0, 0), meta_final);

    strips_solver.resolver();
}
