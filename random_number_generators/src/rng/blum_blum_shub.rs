use super::base::*;
use num_primes::Generator;
use super::linear_congruence_generators::*;
use num_traits::ToPrimitive;

pub struct BlumBlumShub {
    p : u32,
    q : u32,
    m : u64,
    seed : u32,
}

impl RandomNumberGenerator for BlumBlumShub {
    fn set_seed(&mut self,seed : u64) {
        self.generate_p_q_seed(seed);
    }

    fn get_name(self) -> String {
        return String::from("Blum Blum Shub");
    }
}

impl RNG32bitOutput for BlumBlumShub {
    fn next_u32(&mut self) -> u32{
        let mut out : u32 = 0;
        for _i in 0..32 {
            out = (out << 1) | (self.next_bit() & 1_u32);
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
    pub fn set_p_q_seed(&mut self,p:u32,q:u32,seed:u32){
        self.p = p;
        self.q = q;
        self.m = (p * q) as u64;
        self.seed = seed;
    }

    fn next_bit(&mut self ) -> u32 {
        self.seed = ((self.seed as u64 * self.seed as u64) % self.m) as u32;
        return self.seed;
    }

    pub fn generate_p_q_seed(&mut self,seed:u64){
        loop {
            self.p = Generator::new_prime(16).to_u32().unwrap(); //32 bit max to have M not overflow
            if self.p % 4 == 3 {
                break;
            }
        }
        loop {
            self.q = Generator::new_prime(16).to_u32().unwrap(); //32 bit max to have M not overflow
            if self.q % 4 == 3 {
                break;
            }
        }
        self.m = (self.p * self.q) as u64;
        
        let mut rng = park_miller();
        rng.set_seed(seed);
        loop {
            self.seed = rng.next_u32();
            if (self.seed % self.p != 0) && (self.seed % self.q != 0) && (self.seed != 0){
                break;
            }
        }
        
    }
}


pub fn blum_bum_shub() -> BlumBlumShub{
    return BlumBlumShub{seed:7817,p:7603,q:7487,m:0};
}