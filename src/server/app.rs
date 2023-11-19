use crate::server::communicator::Communicator;
use crate::server::thread::tcp_acceptor::TCPAcceptorThread;
use crate::server::thread::tcp_writer::{TCPWriterAPI, TCPWriterThread};
use crate::server::thread::thread_manager::{ThreadManager, ThreadManagerAPI};

pub struct MinersParameters {
    pub host: String,
    pub port: u16,
}

pub struct MinersApp {
    parameters: MinersParameters,
}

impl MinersApp {
    pub fn new(parameters: MinersParameters) -> Self {
        Self { parameters }
    }

    pub fn run(&self) {
        let (thread_manager_api_reader, thread_manager_api_writer) =
            Communicator::new::<ThreadManagerAPI>();
        let (tcp_writer_api_reader, tcp_writer_api_writer) = Communicator::new::<TCPWriterAPI>();

        let tcp_acceptor = TCPAcceptorThread::new(
            self.parameters.host.clone(),
            self.parameters.port,
            thread_manager_api_writer,
        );
        std::thread::spawn(move || tcp_acceptor.execute());

        let mut tcp_writer = TCPWriterThread::new(tcp_writer_api_reader);
        std::thread::spawn(move || tcp_writer.execute());

        ThreadManager::new(thread_manager_api_reader, tcp_writer_api_writer).execute()
    }
}
