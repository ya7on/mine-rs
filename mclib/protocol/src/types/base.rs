pub trait MCType {
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut Vec<u8>) -> Self;

    fn read_byte(src: &mut Vec<u8>) -> u8 {
        src.remove(0)
    }
}
