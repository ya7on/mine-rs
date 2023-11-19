use std::io::Read;

pub trait MCType {
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut dyn Read) -> Self;

    fn read_byte(src: &mut dyn Read) -> u8 {
        let mut buf = [0; 1];
        src.read(&mut buf).unwrap();
        buf[0]
    }
}
