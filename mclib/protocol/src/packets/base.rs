pub trait MCPacket {
    fn packet_id(&self) -> u8;
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut Vec<u8>) -> Self;
}
