use mclib::types::MCVarInt;
use mclib::{MCPacket, MCType};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Packet {
    pub length: i32,
    pub id: i32,
    body: Cursor<Vec<u8>>,
}

impl Packet {
    pub fn init(tcp_buffer: &mut dyn Read) -> Self {
        let length = MCVarInt::unpack(tcp_buffer).into();

        let mut raw_body = Vec::new();
        for _ in 0..length {
            let mut byte_buf = [0; 1];
            tcp_buffer.read_exact(&mut byte_buf).unwrap();
            raw_body.extend(byte_buf);
        }

        let mut body = Cursor::new(raw_body);
        let id = MCVarInt::unpack(&mut body).into();

        Self { length, id, body }
    }

    pub fn parse_packet<P: MCPacket>(&mut self) -> P {
        P::unpack(&mut self.body)
    }
}
