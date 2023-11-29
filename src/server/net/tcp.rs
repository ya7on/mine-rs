use crate::server::packet::Packet;
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
    pub fn read_packet(&mut self) -> Packet {
        Packet::init(&mut self.0)
    }
}

pub struct TCPWrite<TCPInterface: Write>(TCPInterface);

impl<TCPInterface: Write> TCPWrite<TCPInterface> {
    pub fn write_raw(&mut self, data: Vec<u8>) {
        self.0.write_all(&data).unwrap();
    }
}

impl Clone for TCPWrite<NativeWrite> {
    fn clone(&self) -> Self {
        Self(self.0.try_clone().unwrap())
    }
}
