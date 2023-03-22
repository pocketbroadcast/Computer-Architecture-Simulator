mod cpu_state;
mod pin_state;
mod state_machine;

use defer::defer;
use super::cpu_6502::state_machine::{execute, print_cpu_state};

use super::generic::Connection;
use state_machine::StateMachine;

pub struct Component {
    clock_in: Connection<bool>,
    reset_en_in: Connection<bool>,

    data_bus_inout: Connection<u8>,
    rwb_out: Connection<bool>,

    address_bus_out: Connection<u16>,

    state_machine: StateMachine,
}

impl Component {
    pub fn new(
        clock_in: Connection<bool>,
        reset_en_in: Connection<bool>,

        data_bus_inout: Connection<u8>,
        rwb_out: Connection<bool>,

        address_bus_out: Connection<u16>,
    ) -> Self {
        Self {
            clock_in: clock_in,
            reset_en_in: reset_en_in,
            data_bus_inout: data_bus_inout,
            rwb_out: rwb_out,
            address_bus_out: address_bus_out,
            state_machine: StateMachine::new(),
        }
    }

    pub fn tick(&mut self) {
        defer(|| self.write_state());

        let clock = self.clock_in.read_copy();

        if self.state_machine.pin_state.clock == clock {
            // same cycle again? -> abort
            return;
        }

        self.state_machine.pin_state.clock = clock;
        if !clock {
            // falling edge -> no thanks!
            println!("clock: falling-edge");
            return;
        }

        println!("clock: rising-edge");
        self.state_machine.pin_state.reset_en = self.reset_en_in.read_copy();

        if self.state_machine.pin_state.rwb {
            // only update state with external data if in read mode
            self.state_machine.pin_state.data_bus = self.data_bus_inout.read_copy();
        }

        print_cpu_state(&self.state_machine, "before");
        execute(&mut self.state_machine);
        print_cpu_state(&self.state_machine, "after");

        self.write_state();
    }

    fn write_state(&self) {
        self.address_bus_out
            .write_copy(self.state_machine.pin_state.address_bus);

        if !self.state_machine.pin_state.rwb {
            // update external data with state if in write mode
            self.data_bus_inout
                .write_copy(self.state_machine.pin_state.data_bus);
        }

        self.rwb_out.write_copy(self.state_machine.pin_state.rwb);
    }
}
