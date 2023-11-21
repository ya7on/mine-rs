use std::fmt::Debug;
use std::io::Read;

pub trait MCPacket: Debug + Clone {
    fn packet_id(&self) -> i32;
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut dyn Read) -> Self;
}
