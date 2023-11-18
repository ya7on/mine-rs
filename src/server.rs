use crate::tcp_acceptor::TCPAcceptorThread;

pub fn init_threads() {
    // TCP Acceptor thread
    let tcp_acceptor_thread = TCPAcceptorThread::new();
    std::thread::spawn(move || tcp_acceptor_thread.execute());
}
