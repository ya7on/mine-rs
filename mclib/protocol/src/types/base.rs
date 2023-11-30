use std::fmt::Debug;
use std::io::Read;

pub trait MCType: Clone + Debug {
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut dyn Read) -> Self;
}
