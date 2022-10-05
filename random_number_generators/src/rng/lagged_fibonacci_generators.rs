use super::base::*;
use super::linear_congruence_generators::*;

pub struct MitchellMoore {
    pub seed: [u32;55],
    modulo:u64
}
impl RandomNumberGenerator for MitchellMoore {
    fn set_seed(&mut self,seed : u64) {
        let mut rng = park_miller();
        rng.set_seed(seed);
        for i in 0..55 {
            self.seed[i] = rng.next_u32();
        }
    }
}

impl RNG32bitOutput for MitchellMoore {
    fn next_u32(&mut self ) -> u32 {
        let out = lagged_fibo(self.seed[23] as u64,self.seed[54] as u64,self.modulo) as u32;
        self.seed.rotate_right(1);
        self.seed[0] = out;
        return out;
    }
}


impl MitchellMoore{
    pub fn set_modulo(&mut self,modulo : u64){
        self.modulo = modulo;
    }
}

#[inline(always)]
pub fn mitchell_moore() -> MitchellMoore{
    return MitchellMoore{seed:[0;55],modulo:255};
}



fn lagged_fibo(xa:u64,xb:u64,m:u64) ->u64 {
    return(xa + xb) % m;
}

