use std::sync::mpsc::{self};
use std::thread::{self};

pub struct Component {
    address_bus: mpsc::Receiver<u16>,
    data_bus: mpsc::Sender<u8>,

    memory: [u8;0xFFFF+1],
}

impl Component {
    pub fn new(
        address_bus: mpsc::Receiver<u16>,
        data_bus: mpsc::Sender<u8>,
        memory: [u8; 0xFFFF+1]
    ) -> Self {
        Self {
            data_bus: data_bus,
            address_bus: address_bus,
            memory: memory
        }
    }

    pub fn start(self: Self) {
        thread::spawn(move || loop {
            let address = self.address_bus.recv().unwrap();
            self.data_bus.send(self.memory[usize::from(address)]).unwrap();
        });
    }
}
