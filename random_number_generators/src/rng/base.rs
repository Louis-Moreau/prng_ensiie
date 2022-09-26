pub trait RandomNumberGenerator{
    fn set_seed(&mut self,seed : u64);

    fn get_name(self) -> String;
}
pub trait RNG128bitOutput{
    fn next_u128(&mut self ) -> u128;
}

pub trait RNG64bitOutput{
    fn next_u64(&mut self ) -> u64;
}

pub trait RNG32bitOutput{
    fn next_u32(&mut self ) -> u32;
}