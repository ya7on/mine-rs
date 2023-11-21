use std::io::Read;

pub trait MCPacket {
    fn packet_id(&self) -> i32;
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut dyn Read) -> Self;
}
