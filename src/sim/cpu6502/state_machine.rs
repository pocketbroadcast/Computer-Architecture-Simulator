use std::collections::LinkedList;

use super::cpu_state::CPUState;
use super::pin_state::PinState;

pub struct StateMachine {
    cpu_state: CPUState,
    pub pin_state: PinState,

    planned_executions: LinkedList<fn(&mut StateMachine)>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            cpu_state: CPUState::new(),
            pin_state: PinState::new(),
            planned_executions: LinkedList::new(),
        }
    }

    pub fn reset(self: &mut Self) {
        self.cpu_state.reset();
        // do not reset pin state! (as this is given by external components)
        //self.pin_state.reset();
        self.planned_executions.clear();
    }
}

pub fn execute(state_machine: &mut StateMachine) {
    if !state_machine.pin_state.reset_en {
        detail::exec_reset(state_machine);
        return;
    }

    detail::exec_next(state_machine)
}

pub fn print_cpu_state(state_machine: &StateMachine, prefix: &str) {
    println!(
        "{}:\tClock {:5}, Reset {:5}, Addr: 0x{:04x}, RWB {:5}, Data 0x{:02x}, A 0x{:02x}, X 0x{:02x}, Y 0x{:02x}",
        prefix,
        state_machine.pin_state.clock,
        state_machine.pin_state.reset_en,
        state_machine.pin_state.address_bus,
        state_machine.pin_state.rwb,
        state_machine.pin_state.data_bus,
        state_machine.cpu_state.a,
        state_machine.cpu_state.x,
        state_machine.cpu_state.y,
    );
}

mod detail {
    use super::StateMachine;

    pub fn exec_next(state_machine: &mut StateMachine) {
        // no matter what, fall back to default RWB
        state_machine.pin_state.rwb = true;

        if let Some(next) = state_machine.planned_executions.pop_front() {
            next(state_machine);
        } else {
            //println!("No further executions are planned!");
            exec_default(state_machine);
        }
    }

    fn exec_default(state_machine: &mut StateMachine) {
        // fetch next instruction
        let next_instruction = state_machine.pin_state.data_bus;
        //println!("Next instruction fetched is 0x{:02x}", next_instruction);

        // decode instruction
        //...
        match next_instruction {
            0xA9 => {
                // LDA immediate
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.a = sm.pin_state.data_bus;

                    inc_pc_and_address_it(sm);
                });

                inc_pc_and_address_it(state_machine);
            }
            0x29 => {
                // AND immediate
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.a &= sm.pin_state.data_bus;

                    inc_pc_and_address_it(sm);
                });

                inc_pc_and_address_it(state_machine);
            }
            0x0A => {
                // ASL A
                state_machine.cpu_state.a = state_machine.cpu_state.a << 1;
                inc_pc_and_address_it(state_machine);
            }
            0x4C => {
                // JMP $0x0000 (absolute)
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.internal &= 0xFF00;
                    sm.cpu_state.internal |= u16::from(sm.pin_state.data_bus);

                    inc_pc_and_address_it(sm);
                });
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.internal &= 0x00FF;
                    sm.cpu_state.internal |= u16::from(sm.pin_state.data_bus) << 8;

                    set_pc_and_address_it(sm, sm.cpu_state.internal);
                });

                inc_pc_and_address_it(state_machine);
            }
            0x8D => {
                // STA $0x0000 (store accumulator absolute)
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.internal &= 0xFF00;
                    sm.cpu_state.internal |= u16::from(sm.pin_state.data_bus);

                    inc_pc_and_address_it(sm);
                });
                state_machine.planned_executions.push_back(|sm| {
                    sm.cpu_state.internal &= 0x00FF;
                    sm.cpu_state.internal |= u16::from(sm.pin_state.data_bus) << 8;

                    sm.pin_state.address_bus = sm.cpu_state.internal;
                    sm.pin_state.data_bus = sm.cpu_state.a;
                    sm.pin_state.rwb = false;

                    inc_pc(sm);
                });

                inc_pc_and_address_it(state_machine);
            }

            _ => {
                println!("unknown instruction -> reset CPU!");
                exec_reset(state_machine);
            }
        }

        // inc_pc(&mut state_machine.cpu_state);
        // state_machine.pin_state.address_bus = state_machine.cpu_state.pc;
    }

    pub fn exec_reset(state_machine: &mut StateMachine) {
        println!("Reset processor");
        state_machine.reset();

        state_machine.pin_state.address_bus = 0xfffc;
        //state_machine.planned_executions.push(|sm| {
        //   sm.pin_state.address_bus = 0xfffc;
        //});
        state_machine.planned_executions.push_back(|sm| {
            sm.cpu_state.pc &= 0xFF00;
            sm.cpu_state.pc |= u16::from(sm.pin_state.data_bus);
            sm.pin_state.address_bus = 0xfffd;
        });
        state_machine.planned_executions.push_back(|sm| {
            sm.cpu_state.pc &= 0x00FF;
            sm.cpu_state.pc |= u16::from(sm.pin_state.data_bus) << 8;
            sm.pin_state.address_bus = sm.cpu_state.pc;
        });
    }

    fn set_pc_and_address_it(state_machine: &mut StateMachine, new_pc: u16) {
        state_machine.cpu_state.pc = new_pc;
        state_machine.pin_state.address_bus = state_machine.cpu_state.pc;
    }

    fn inc_pc_and_address_it(state_machine: &mut StateMachine) {
        inc_pc(state_machine);
        state_machine.pin_state.address_bus = state_machine.cpu_state.pc;
    }

    fn inc_pc(state_machine: &mut StateMachine) {
        if state_machine.cpu_state.pc < std::u16::MAX {
            state_machine.cpu_state.pc += 1;
        } else {
            println!("Program counter overflow!!!");
            state_machine.cpu_state.pc = 0;
        }
    }
}
