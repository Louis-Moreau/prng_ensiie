#![allow(dead_code)]
mod rng;
mod analysis;
use rng::{linear_congruence_generators::*, base::*, lagged_fibonacci_generators::*,blum_blum_shub::*};
use rand_xoshiro::rand_core::{SeedableRng,RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::{Instant, Duration};
use ndarray::{aview2, arr2, Array2};
use ndarray_stats::CorrelationExt;

use crate::analysis::analyse;

const BENCHMARK_SIZE : usize = 10_000_000;

const SEED : u64 = 224 ;



fn main() {


    let mut park_millerr = park_miller();
    park_millerr.set_seed(SEED);

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = park_millerr.next_u32();
    }
    end_benchmark_and_print(bench, "Park Miller");
    analyse(&vec_buffer,"Park Miller");


    let mut park_miller2 = park_miller();
    park_miller2.set_seed(SEED+1);
    let mut arr = Array2::<f64>::zeros((2, BENCHMARK_SIZE));
    for i in 0..BENCHMARK_SIZE {
        arr[[0,i]] = normalize(park_millerr.next_u32());
        arr[[1,i]] = normalize(park_miller2.next_u32());
    }
    let corr = arr.pearson_correlation().unwrap();
    println!("correlation :{}",corr[[0,1]]);






    let mut mitchell_mooree = mitchell_moore();
    mitchell_mooree.set_seed(SEED);
    mitchell_mooree.set_modulo(2_u64.pow(32));

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = mitchell_mooree.next_u32();
    }
    end_benchmark_and_print(bench, "Mitchell Moore");
    analyse(&vec_buffer,"Mitchell Moore");

    let mut mitchell_moore2 = mitchell_moore();
    mitchell_moore2.set_seed(SEED+1);
    let mut arr = Array2::<f64>::zeros((2, BENCHMARK_SIZE));
    for i in 0..BENCHMARK_SIZE {
        arr[[0,i]] = normalize(mitchell_mooree.next_u32());
        arr[[1,i]] = normalize(mitchell_moore2.next_u32());
    }
    let corr = arr.pearson_correlation().unwrap();
    println!("correlation :{}",corr[[0,1]]);





    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(SEED);

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = xoshiro.next_u32();
    }
    end_benchmark_and_print(bench, "Xoshiro256++");
    analyse(&vec_buffer,"Xoshiro256++");

    let mut xoshiro2 = Xoshiro256PlusPlus::seed_from_u64(SEED+1);
    let mut arr = Array2::<f64>::zeros((2, BENCHMARK_SIZE));
    for i in 0..BENCHMARK_SIZE {
        arr[[0,i]] = normalize(xoshiro.next_u32());
        arr[[1,i]] = normalize(xoshiro2.next_u32());
    }
    let corr = arr.pearson_correlation().unwrap();
    println!("correlation :{}",corr[[0,1]]);



    


    
    let mut blum_bum_shubb = blum_bum_shub();
    //blum_bum_shub.generate_p_q_seed(SEED); not working , do not use

    let mut vec_buffer = vec![0u32;BENCHMARK_SIZE];

    let bench = start_benchmark();
    for i in 0..BENCHMARK_SIZE {
        vec_buffer[i] = blum_bum_shubb.next_u32();
    }
    end_benchmark_and_print(bench, "Blum Blum Shub");
    analyse(&vec_buffer,"Blum Blum Shub");

    let mut blum_bum_shub2 = Xoshiro256PlusPlus::seed_from_u64(SEED+1);
    let mut arr = Array2::<f64>::zeros((2, BENCHMARK_SIZE));
    for i in 0..BENCHMARK_SIZE {
        arr[[0,i]] = normalize(blum_bum_shubb.next_u32());
        arr[[1,i]] = normalize(blum_bum_shub2.next_u32());
    }
    let corr = arr.pearson_correlation().unwrap();
    println!("correlation :{}",corr[[0,1]]);




}

fn start_benchmark() -> Instant{
    return Instant::now();
}

fn end_benchmark_and_print(inst : Instant,name : &str) {
    let time = inst.elapsed();
    println!("Time for {} iter of {}: {:?} , {:?} per iteration",BENCHMARK_SIZE , name , time, time/ BENCHMARK_SIZE as u32);
    
}

fn normalize(input: u32) -> f64{
    return input as f64 / u32::MAX as f64;
}


/*
standard minimal DONE
michel et moore (fibo retardé) DONE
standard utilisé par le language DONE
blum blum shub DONE
congruence linéaire DONE


*/
