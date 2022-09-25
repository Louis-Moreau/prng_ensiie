use super::base::*;

pub struct ParkMiller {
    seed : u64,
}
impl RandomNumberGenerator for ParkMiller {
    fn set_seed(&mut self,seed : u64) {
        self.seed = seed;
    }

    fn next(&mut self ) -> u64 {
        self.seed = lcg(2u64.pow(32)-1,16807,0,self.seed);
        return self.seed;
    }
}
#[inline(always)]
pub fn park_miller() -> ParkMiller{
    return ParkMiller{seed:0};
}

pub struct KnuthLewis {
    seed : u64,
}
impl RandomNumberGenerator for KnuthLewis {
    fn set_seed(&mut self,seed : u64) {
        self.seed = seed;
    }

    fn next(&mut self ) -> u64 {
        self.seed = lcg(2u64.pow(32),1664525,1013904223,self.seed);
        return self.seed;
    }
}
#[inline(always)]
pub fn knuth_lewis() -> KnuthLewis{
    return KnuthLewis{seed:0};
}


pub struct Marsaglia {
    seed : u64,
}
impl RandomNumberGenerator for Marsaglia {
    fn set_seed(&mut self,seed : u64) {
        self.seed = seed;
    }

    fn next(&mut self ) -> u64 {
        self.seed = lcg(2u64.pow(32),69069,0,self.seed);
        return self.seed;
    }
}
#[inline(always)]
pub fn marsaglia() -> Marsaglia{
    return Marsaglia{seed:0};
}

pub struct LaveuxJenssens {
    seed : u64,
}
impl RandomNumberGenerator for LaveuxJenssens {
    fn set_seed(&mut self,seed : u64) {
        self.seed = seed;
    }

    fn next(&mut self ) -> u64 {
        self.seed = lcg(2u64.pow(48),31167285,1,self.seed);
        return self.seed;
    }
}
#[inline(always)]
pub fn lavaux_jenssens() -> LaveuxJenssens{
    return LaveuxJenssens{seed:0};
}

/*#[inline(always)]
pub fn haynes(x:u128) -> u128 {
    return lcg_128(2^64,6364136223846793005,1,x);
}*/

#[inline(always)]
fn lcg(m:u64,a:u64,c:u64,x:u64) -> u64 {
    return (a * x + c) % m;
}