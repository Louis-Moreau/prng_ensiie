pub trait RandomNumberGenerator{
    fn set_seed(&mut self,seed : u64);
    fn next(&mut self ) -> u64;
}