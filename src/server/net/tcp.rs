use mclib::types::MCVarInt;
use mclib::{MCPacket, MCType};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub type NativeTCP = TcpListener;
pub type NativeRead = TcpStream;
pub type NativeWrite = TcpStream;
pub type NativeAddr = SocketAddr;

pub struct TCPServer<TCPInterface>(TCPInterface);

impl TCPServer<NativeTCP> {
    pub fn new(host: String, port: u16) -> Self {
        debug!("new NativeTCP listener {}:{}", host, port);
        Self(TcpListener::bind((host, port)).unwrap())
    }

    pub fn accept(&self) -> (TCPRead<NativeRead>, TCPWrite<NativeWrite>, NativeAddr) {
        let (stream, addr) = self.0.accept().unwrap(); // TODO error handling (may be panic here?);
        (TCPRead(stream.try_clone().unwrap()), TCPWrite(stream), addr)
    }
}

pub struct TCPRead<TCPInterface: Read>(TCPInterface);

impl<TCPInterface: Read> TCPRead<TCPInterface> {
    pub fn read_raw_packet(&mut self) -> (i32, Vec<u8>) {
        let packet_length = MCVarInt::unpack(&mut self.0).into();
        let mut packet_buffer = Vec::new();
        for _ in 0..packet_length {
            let mut buf = [0; 1];
            self.0.read(&mut buf).unwrap();
            packet_buffer.extend(buf);
        }
        (packet_length, packet_buffer)
    }

    pub fn unpack_packet_id(buffer: &mut dyn Read) -> i32 {
        MCVarInt::unpack(buffer).into()
    }

    pub fn read_specific_packet<P: MCPacket>(&mut self) -> (i32, i32, P) {
        let (packet_length, raw_packet) = self.read_raw_packet();
        let mut packet_cursor = std::io::Cursor::new(raw_packet);
        let packet_id = Self::unpack_packet_id(&mut packet_cursor);
        let packet = P::unpack(&mut packet_cursor);
        (packet_length, packet_id, packet)
    }
}

pub struct TCPWrite<TCPInterface: Write>(TCPInterface);

impl<TCPInterface: Write> TCPWrite<TCPInterface> {
    pub fn write_raw(&mut self, data: Vec<u8>) {
        self.0.write(&data).unwrap();
    }
}

impl Clone for TCPWrite<NativeWrite> {
    fn clone(&self) -> Self {
        Self(self.0.try_clone().unwrap())
    }
}
