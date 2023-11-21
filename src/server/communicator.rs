use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct Communicator;

impl Communicator {
    pub fn create<API>() -> (ReadCommunicator<API>, WriteCommunicator<API>) {
        let (write, read) = mpsc::channel();
        (ReadCommunicator(read), WriteCommunicator(write))
    }
}

pub struct ReadCommunicator<API>(Receiver<API>);

impl<API> ReadCommunicator<API> {
    pub fn recv(&self) -> API {
        self.0.recv().unwrap()
    }
}

#[derive(Clone)]
pub struct WriteCommunicator<API>(Sender<API>);

impl<API> WriteCommunicator<API> {
    pub fn send(&self, msg: API) {
        self.0.send(msg).unwrap()
    }
}
