#![allow(dead_code)]
mod rng;
mod analysis;
use rng::{linear_congruence_generators::*, base::*, lagged_fibonacci_generators::*,blum_blum_shub::*};
use rand_xoshiro::rand_core::{SeedableRng,RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::{Instant, Duration};

use crate::analysis::analyse;

const BENCHMARK_SIZE : usize = 10_000_000;

const SEED : u64 = 224 ;



fn main() {


    let mut park_miller = park_miller();
    park_miller.set_seed(SEED);

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = park_miller.next_u32();
    }
    end_benchmark_and_print(bench, "Park Miller");
    analyse(&vec_buffer,"Park Miller");



    let mut mitchell_moore = mitchell_moore();
    mitchell_moore.set_seed(SEED);
    mitchell_moore.set_modulo(2_u64.pow(32));

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = mitchell_moore.next_u32();
    }
    end_benchmark_and_print(bench, "Mitchell Moore");
    analyse(&vec_buffer,"Mitchell Moore");





    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(SEED);

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = xoshiro.next_u32();
    }
    end_benchmark_and_print(bench, "Xoshiro256++");
    analyse(&vec_buffer,"Xoshiro256++");


    


    
    let mut blum_bum_shub = blum_bum_shub();
    //blum_bum_shub.generate_p_q_seed(10000000);

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = blum_bum_shub.next_u32();
    }
    end_benchmark_and_print(bench, "Blum Blum Shub");
    analyse(&vec_buffer,"Blum Blum Shub");



}

fn start_benchmark() -> Instant{
    return Instant::now();
}

fn end_benchmark_and_print(inst : Instant,name : &str) {
    let time = inst.elapsed();
    println!("Time for {} iter of {}: {:?} , {:?} per iteration",BENCHMARK_SIZE , name , time, time/ BENCHMARK_SIZE as u32);
    
}


/*
standard minimal DONE
michel et moore (fibo retardé) DONE
standard utilisé par le language DONE
blum blum shub DONE
congruence linéaire DONE


*/
