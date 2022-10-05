use super::base::*;
use num_primes::Generator;
use super::linear_congruence_generators::*;
use num_traits::ToPrimitive;
use rand_xoshiro::rand_core::{SeedableRng,RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct BlumBlumShub {
    p : u64,
    q : u64,
    m : u128,
    seed : u128,
}

impl RandomNumberGenerator for BlumBlumShub {
    fn set_seed(&mut self,seed : u64) {
        self.generate_p_q_seed(seed);
    }
}

impl RNG32bitOutput for BlumBlumShub {
    fn next_u32(&mut self) -> u32{
        let mut out : u32 = 0;
        for _i in 0..32 {
            out = (out << 1) | (self.next_bit()as u32 & 1_u32);
        }
        return out;
    }
}

impl RNG64bitOutput for BlumBlumShub {
    fn next_u64(&mut self) -> u64{
        let mut out : u64 = 0;
        for _i in 0..64 {
            out = (out << 1) | (self.next_bit()as u64 & 1_u64);
        }
        return out;
    }
}

impl RNG128bitOutput for BlumBlumShub {
    fn next_u128(&mut self) -> u128{
        let mut out : u128 = 0;
        for _i in 0..128 {
            out = (out << 1) | (self.next_bit()as u128 & 1_u128);
        }
        return out;
    }
}


impl BlumBlumShub {
    pub fn set_p_q_seed(&mut self,p:u64,q:u64,seed:u64){
        self.p = p;
        self.q = q;
        self.m = (p as u128 * q as u128 );
        self.seed = seed as u128; 
    }

    fn next_bit(&mut self ) -> u32 {
        self.seed = ((self.seed as u128 * self.seed as u128) % self.m) as u128;
        return self.seed as u32;
    }


    //this does not work and break after ~100k iterations
    pub fn generate_p_q_seed(&mut self,seed:u64){
        loop {
            self.p = Generator::safe_prime(64).to_u64().unwrap(); //32 bit max to have M not overflow
            if self.p % 4 == 3 {
                break;
            }
        }
        loop {
            self.q = Generator::safe_prime(64).to_u64().unwrap(); //32 bit max to have M not overflow
            if self.q % 4 == 3 {
                break;
            }
        }
        self.m = self.p as u128 * self.q as u128;
        
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        loop {
            self.seed = rng.next_u64() as u128;
            if (self.seed % self.p as u128 != 0) && (self.seed % self.q as u128 != 0) && (self.seed != 0){
                break;
            }
        }
        
    }
}


pub fn blum_bum_shub() -> BlumBlumShub{
    return BlumBlumShub{seed:4121221139u128,p:30000000091u64,q:40000000003u64,m:1200000003730000000273u128};
}