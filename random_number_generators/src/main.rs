#![allow(dead_code)]
mod rng;
use rng::{linear_congruence_generators::*, base::*, lagged_fibonacci_generators::*,blum_blum_shub::*};
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
    println!("mitchellmoore : {}",mm.next_u64());
    println!("mitchellmoore : {}",mm.next_u64());
    for _i in 0..200 {
        mm.next_u64();
    }
    println!("mitchellmoore : {}",mm.next_u64());
   

/* 
    let mut xm = Xoshiro256PlusPlus::seed_from_u64(223);
    println!("xoshiro256 : {}",xm.next_u64());
    println!("xoshiro256 : {}",xm.next_u64());
    println!("xoshiro256 : {}",xm.next_u64());*/

    let mut test1 : [u64; 10_000_000] = [0;10_000_000];
    let duration = Instant::now();
    
    let mut benchmark = mitchell_moore();
    benchmark.set_seed(15);
    benchmark.set_modulo(2_u64.pow(32));


    for i in 0..10_000_000 {
        test1[i] = benchmark.next_u64();
    }

    let end = duration.elapsed();

    println!("time Mitchel&Moore: {:?}", end);
    println!("per iter Mitchel: {:?}", end/10_000_000);

    let mut test2 : [u64; 10_000_000] = [0;10_000_000];

    
    let mut benchmark = blum_bum_shub();
    benchmark.generate_p_q_seed(15);

    let duration = Instant::now();

    for i in 0..10_000_000 {
        test2[i] = benchmark.next_u64();
    }

    let end = duration.elapsed();

    println!("BlumBlumShub : {}",benchmark.next_u64());
    println!("BlumBlumShub : {}",benchmark.next_u64());
    println!("BlumBlumShub : {}",benchmark.next_u64());

    println!("time BlumBlumShub: {:?}", end);
    println!("per iter BlumBlumShub: {:?}", end/10_000_000);



}


/*
standard minimal DONE
michel et moore (fibo retardé) DONE
standard utilisé par le language DONE
blum blum shub
congruence linéaire DONE


*/
