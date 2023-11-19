use crate::server::communicator::WriteCommunicator;
use crate::server::net::tcp::{NativeTCP, TCPServer};
use crate::server::thread::thread_manager::ThreadManagerAPI;

pub struct TCPAcceptorThread {
    listener: TCPServer<NativeTCP>,
    thread_manager_api: WriteCommunicator<ThreadManagerAPI>,
}

impl TCPAcceptorThread {
    pub fn new(
        host: String,
        port: u16,
        thread_manager_api: WriteCommunicator<ThreadManagerAPI>,
    ) -> Self {
        let listener = TCPServer::new(host, port);
        Self {
            listener,
            thread_manager_api,
        }
    }

    pub fn execute(&self) {
        loop {
            let (tcp_read, tcp_write, tcp_addr) = self.listener.accept();

            self.thread_manager_api
                .send(ThreadManagerAPI::NewConnection {
                    tcp_read,
                    tcp_write,
                    tcp_addr,
                });
        }
    }
}
