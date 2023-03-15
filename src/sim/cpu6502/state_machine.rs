use super::cpu_state::CPUState;
use super::pin_state::PinState;

pub struct StateMachine {
    cpu_state: CPUState,
    pub pin_state: PinState,

    planned_executions: Vec<fn(&mut StateMachine)>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            cpu_state: CPUState::new(),
            pin_state: PinState::new(),
            planned_executions: Vec::new(),
        }
    }

    pub fn reset(self: &mut Self) {
        self.cpu_state.reset();
        self.pin_state.reset();
        self.planned_executions.clear();
    }
}

pub fn execute(state_machine: &mut StateMachine) {
    println!(
        "Clock {:?}, Reset {:?}, Addr: 0x{:04x}, Data 0x{:02x}",
        state_machine.pin_state.clock,
        state_machine.pin_state.reset_en,
        state_machine.pin_state.address_bus,
        state_machine.pin_state.data_bus
    );

    if !state_machine.pin_state.reset_en {
        detail::exec_reset(state_machine);
    }

    detail::exec_next(state_machine)
}

mod detail {
    use super::CPUState;
    use super::StateMachine;

    pub fn exec_next(state_machine: &mut StateMachine) {
        if let Some(next) = state_machine.planned_executions.pop() {
            next(state_machine);
        } else {
            println!("No further executions are planned!");
        }

        exec_default(state_machine);
    }

    pub fn exec_default(state_machine: &mut StateMachine) {
        // fetch next instruction
        let next_instruction = state_machine.pin_state.data_bus;
        println!("Next instruction fetched is 0x{:02x}", next_instruction);

        // decode instruction
        //...
        inc_pc(&mut state_machine.cpu_state);
        state_machine.pin_state.address_bus = state_machine.cpu_state.pc;
    }

    pub fn exec_reset(state_machine: &mut StateMachine) {
        println!("Reset processor");
        state_machine.reset();
    }

    fn inc_pc(cpu_state: &mut CPUState) {
        if cpu_state.pc < std::u16::MAX {
            cpu_state.pc += 1;
        } else {
            println!("Program counter overflow!!!");
            cpu_state.pc = 0;
        }
    }
}
