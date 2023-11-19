use crate::server::communicator::{ReadCommunicator, WriteCommunicator};
use crate::server::net::tcp::{NativeAddr, NativeRead, NativeWrite, TCPRead, TCPWrite};
use crate::server::thread::tcp_listener::TCPListenerThread;
use crate::server::thread::tcp_writer::TCPWriterAPI;
use std::collections::HashMap;
use std::thread::JoinHandle;

pub enum ThreadManagerAPI {
    NewConnection {
        tcp_read: TCPRead<NativeRead>,
        tcp_write: TCPWrite<NativeWrite>,
        tcp_addr: NativeAddr,
    },
}

pub struct ThreadManager {
    thread_manager_api: ReadCommunicator<ThreadManagerAPI>,
    tcp_writer_api: WriteCommunicator<TCPWriterAPI>,
    tcp_listeners: HashMap<u128, JoinHandle<()>>,
}

impl ThreadManager {
    pub fn new(
        thread_manager_api: ReadCommunicator<ThreadManagerAPI>,
        tcp_writer_api: WriteCommunicator<TCPWriterAPI>,
    ) -> Self {
        Self {
            thread_manager_api,
            tcp_writer_api,
            tcp_listeners: HashMap::new(),
        }
    }

    pub fn execute(&mut self) {
        loop {
            match self.thread_manager_api.recv() {
                ThreadManagerAPI::NewConnection {
                    tcp_read,
                    tcp_write,
                    tcp_addr,
                } => {
                    debug!("New connection {}", tcp_addr);

                    let uid = uuid::Uuid::new_v4().as_u128();
                    self.tcp_writer_api
                        .send(TCPWriterAPI::NewConnection { tcp_write, uid });

                    let mut tcp_listener =
                        TCPListenerThread::new(uid, tcp_read, self.tcp_writer_api.clone());
                    self.tcp_listeners
                        .insert(uid, std::thread::spawn(move || tcp_listener.execute()));
                }
            }
        }
    }
}
