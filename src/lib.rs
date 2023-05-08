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

pub fn run_hanoi(discos: i8) {
    hanoi(discos);
}

pub fn bench_hanoi() {
    const TIMES: u128 = 2;
    const DISCOS: i8 = 14;
    let mut total_time: u128 = 0;
    let mut best_time: u128 = u128::MAX;
    let mut worst_time: u128 = 0;

    hanoi(DISCOS);
    for _ in 0..TIMES {
        let actual_time = hanoi(DISCOS).expect("Fallo en Strips");

        total_time += actual_time;

        if actual_time > worst_time {
            worst_time = actual_time;
        }

        if actual_time < best_time {
            best_time = actual_time;
        }
    }

    println!("Media: {}", total_time / TIMES);
    println!("\tBest case: {}", best_time);
    println!("\tWorst case: {}", worst_time);
}

pub fn hanoi(discos: i8) -> Option<u128> {
    let mut estado_actual = Vec::new();
    //const discos: i8 = 10;

    estado_actual.push(Meta::Despejado(1));
    estado_actual.push(Meta::Despejado(-2));
    estado_actual.push(Meta::Despejado(-3));

    for disco_actual in 2..=discos {
        for i in 1..disco_actual {
            estado_actual.push(Meta::Menor(i, disco_actual));
        }
    }

    for i in 1..=discos {
        for varilla in -3..0 {
            estado_actual.push(Meta::Menor(i, varilla));
        }
    }

    for i in 1..discos {
        estado_actual.push(Meta::Sobre(i, i + 1));
    }
    estado_actual.push(Meta::Sobre(discos, -1));

    // Objetivos
    let mut objetivos = Vec::new();
    for i in 1..discos {
        objetivos.push(Stackeable::Objetivo(Meta::Sobre(i, i + 1)));
    }
    objetivos.push(Stackeable::Objetivo(Meta::Sobre(discos, -3)));

    // Estado final
    let mut meta_final = Vec::new();
    for i in 1..discos {
        meta_final.push(Meta::Sobre(i, i + 1));
    }
    meta_final.push(Meta::Sobre(discos, -3));

    let estado_inicial = StripsState::new(estado_actual, objetivos, (discos - 1) as usize);
    let strips_solver = Strips::new(estado_inicial, Apilar::new(0, 0, 0), meta_final, discos);

    strips_solver.resolver()
}
