#![allow(dead_code)]
mod rng;
use rng::{linear_congruence_generators::*, base::*, lagged_fibonacci_generators::*};
use rand_xoshiro::rand_core::{SeedableRng,RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::Instant;



fn main() {
    /*let mut pm = park_miller();
    pm.set_seed(223);
    println!("parkmiller : {}", pm.next());
    println!("parkmiller : {}", pm.next());
    println!("parkmiller : {}", pm.next());
*/
    let mut mm = mitchell_moore();

    mm.set_seed(223);
    mm.set_modulo(2_u64.pow(32));
    println!("mitchellmoore : {}",mm.next());
    println!("mitchellmoore : {}",mm.next());
    for _i in 0..200 {
        mm.next();
    }
    println!("mitchellmoore : {}",mm.next());
   

/* 
    let mut xm = Xoshiro256PlusPlus::seed_from_u64(223);
    println!("xoshiro256 : {}",xm.next_u64());
    println!("xoshiro256 : {}",xm.next_u64());
    println!("xoshiro256 : {}",xm.next_u64());*/


    let duration = Instant::now();
    
    let mut benchmark = mitchell_moore();
    benchmark.set_seed(15);
    benchmark.set_modulo(2_u64.pow(32));

    for _i in 0..20_000_000 {
        benchmark.next();
    }

    let end = duration.elapsed();

    println!("time per ited: {:?}", end);



}


/*
standard minimal DONE
michel et moore (fibo retardé) DONE
standard utilisé par le language DONE
blum blum shub
congruence linéaire DONE


*/
