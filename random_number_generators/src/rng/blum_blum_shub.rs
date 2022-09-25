use super::base::*;

pub struct BlumBlumShub {
    p : u64,
    q : u64,
    m : u64,
    seed : u64,
}
impl RandomNumberGenerator for BlumBlumShub {
    fn set_seed(&mut self,seed : u64) {
        self.seed = seed;
    }

    fn next(&mut self ) -> u64 {
        self.seed = lcg(2u64.pow(32),1664525,1013904223,self.seed);
        return self.seed;
    }
}
#[inline(always)]
pub fn blum_bum_shub() -> BlumBlumShub{
    return BlumBlumShub{seed:0};
}