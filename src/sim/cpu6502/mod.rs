mod cpu_state;
mod pin_state;
mod state_machine;

use std::sync::mpsc::{self};
use std::thread::{self};

use crate::sim::cpu6502::state_machine::execute;

use super::generic::receive_nonblocking;
use state_machine::StateMachine;

pub struct Component {
    clock: mpsc::Receiver<bool>,
    reset_en: mpsc::Receiver<bool>,

    data_bus: mpsc::Receiver<u8>,
    address_bus: mpsc::Sender<u16>,

    state_machine: StateMachine
}

impl Component {
    pub fn new(
        clock: mpsc::Receiver<bool>,
        reset_en: mpsc::Receiver<bool>,

        data_bus: mpsc::Receiver<u8>,
        address_bus: mpsc::Sender<u16>,
    ) -> Self {
        Self {
            clock: clock,
            reset_en: reset_en,
            data_bus: data_bus,
            address_bus: address_bus,
            state_machine: StateMachine::new()
        }
    }

    pub fn start(mut self: Self) {
        thread::spawn(move || loop {
            let clock = self.clock.recv().unwrap();

            self.state_machine.pin_state.clock = clock;
            if !clock {
                println!("clock: falling-edge");
                continue;
            }

            println!("clock: rising-edge");
            self.state_machine.pin_state.reset_en = receive_nonblocking(&self.reset_en, &self.state_machine.pin_state.reset_en);
            self.state_machine.pin_state.data_bus = receive_nonblocking(&self.data_bus, &self.state_machine.pin_state.data_bus);

            execute(&mut self.state_machine);

            self.address_bus.send(self.state_machine.pin_state.address_bus).unwrap();
        });
    }
}
