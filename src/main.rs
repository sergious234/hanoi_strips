#![forbid(unsafe_code)]

use hanoi::*;
use std::env;

pub mod accion;
pub mod stackeable;
pub mod strips;
pub mod stripsstate;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        hanoi(11);
    } else if args.len() == 2 {
        let start: i8 = args.get(1).unwrap().parse().expect("Eso no es un numero!");
        print!("Discos: {} | ", start);
        hanoi(start);
        hanoi(start);
    } else {
        let start: i8 = args.get(1).unwrap().parse().expect("Eso no es un numero!");
        let end: i8 = args.get(2).unwrap().parse().expect("Eso no es un numero!");

        for i in start..=end {
            print!("Discos: {} | ", i);
            run_hanoi(i)
        }
    }
}
