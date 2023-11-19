use mclib::{MCPacket, MCType, MCVarInt};
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
    pub fn read_packet_header(&mut self, compression_treshold: Option<i32>) -> (i32, i32) {
        if let Some(_compression) = compression_treshold {
            todo!()
        }
        let packet_length = MCVarInt::unpack(&mut self.0);
        let packet_id = MCVarInt::unpack(&mut self.0);
        (packet_length.into(), packet_id.into())
    }

    pub fn read_specific_packet<P: MCPacket>(&mut self, compression_treshold: Option<i32>) -> P {
        let (packet_length, packet_id) = self.read_packet_header(compression_treshold);
        P::unpack(&mut self.0)
    }
}

pub struct TCPWrite<TCPInterface: Write>(TCPInterface);

impl<TCPInterface: Write> TCPWrite<TCPInterface> {
    pub fn write(&self) {
        todo!()
    }
}

impl Clone for TCPWrite<NativeWrite> {
    fn clone(&self) -> Self {
        Self(self.0.try_clone().unwrap())
    }
}
