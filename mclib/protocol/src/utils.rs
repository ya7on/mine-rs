use std::io::Read;

pub trait TcpUtils {
    fn read_byte(&mut self) -> u8;
}

impl TcpUtils for dyn Read + '_ {
    fn read_byte(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).unwrap();
        buf[0]
    }
}
