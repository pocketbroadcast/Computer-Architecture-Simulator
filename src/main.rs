mod sim;

use crate::sim::run_test;

struct CPUState {
    a: u8,
    x: u8,
    y: u8,

    sp: u16, // 8 bit setable with 0x0100 memory offset
    pc: u16,
}

impl CPUState {
    fn new() -> Self {
        let mut new_state = Self {
            a: 0,
            x: 0,
            y: 0,

            pc: 0x0000, // gets loaded from FFFC (low byte) FFFD (high byte) -> little endian,
            sp: 0x01FF,
        };

        new_state.reset();

        new_state
    }

    fn reset(self: &mut Self) -> &mut Self {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0x0000; // gets loaded from FFFC (low byte) FFFD (high byte) -> little endian,
        self.sp = 0x01FF;

        self
    }
}

// trait for State for new and reset

#[derive(Debug, Clone, Copy)]
enum SignalState {
    HIGH,
    LOW,
    FLOATING,
}

impl Default for SignalState {
    fn default() -> Self {
        SignalState::FLOATING
    }
}

struct Signal<'a, T> {
    state: T,
    on_change: Vec<Box<dyn FnMut(T) -> () + 'a>>,
}

impl<'a, T: Default + Copy> Signal<'a, T> {
    fn new() -> Self {
        let new_signal = Self {
            state: Default::default(),
            on_change: Vec::new(),
        };

        new_signal
    }

    fn register_on_change(self: &mut Self, on_change_callback: impl FnMut(T) + 'a) {
        self.on_change.push(Box::new(on_change_callback));
    }

    fn set(self: &mut Self, value: T) {
        self.state = value;
        for on_change_handler in self.on_change.iter_mut() {
            on_change_handler(self.state);
        }
    }
}

struct PinState<'a> {
    reset_en: Signal<'a, SignalState>,
    clock: Signal<'a, SignalState>,

    data_bus: Signal<'a, u8>,
    address_bus: Signal<'a, u16>,
}

impl<'a> PinState<'a> {
    fn new() -> Self {
        Self {
            clock: Signal::new(),
            reset_en: Signal::new(),
            data_bus: Signal::new(),
            address_bus: Signal::new(),
        }
    }
}

struct CPU6502<'a> {
    state: CPUState,
    pin_state: PinState<'a>,

    next_execution: Vec<fn(&mut CPU6502<'a>)>,
}

impl<'a> CPU6502<'a> {
    fn new() -> Self {
        let new_statemachine = Self {
            state: CPUState::new(),
            pin_state: PinState::new(),
            next_execution: Vec::new(),
        };

        new_statemachine
    }

    fn execReset(self: &mut Self) -> &mut Self {
        println!("Reset");

        self.state.reset();
        self.next_execution.clear();

        self
    }

    fn clockTick(self: &mut Self, rising_edge: bool) -> () {
        if rising_edge {
            //println!("rising edge!");
            self.next();
        }
    }

    fn inc_pc(self: &mut Self) {
        if self.state.pc < std::u16::MAX {
            self.state.pc += 1;
        } else {
            self.state.pc = 0;
        }
    }

    fn next(self: &mut Self) -> () {
        if let SignalState::LOW = self.pin_state.reset_en.state {
            self.execReset();

            return;
        }

        println!(
            "Status: PC 0x{:04x}, Addr: 0x{:04x}, Data 0x{:02x}",
            self.state.pc, self.pin_state.address_bus.state, self.pin_state.data_bus.state
        );
        // do execution
        // println!("do smth productive!");
        self.next_execution.push(|cpu: &mut Self| -> () {
            // println!("do something on cpu! {:?}", cpu.state.pc);
            cpu.inc_pc();
            cpu.pin_state.address_bus.set(cpu.state.pc);
        });

        if let Some(next) = self.next_execution.pop() {
            next(self);
        }

        return;
    }
}

struct MemorySim<'a> {
    mem: [u8; 0xFFFF + 1],

    address_bus: Signal<'a, u16>,
    data_bus: Signal<'a, u8>,
}

impl<'a> MemorySim<'a> {
    fn new(init_mem: [u8; 0xFFFF + 1]) -> Self {
        let new_memory_sim = Self {
            mem: init_mem,
            data_bus: Signal::new(),
            address_bus: Signal::new(),
        };

        new_memory_sim
    }

    fn set_address_bus(self: &mut Self, address_bus: &Signal<'a, u16>) -> () {
        self.address_bus.set(address_bus.state);
        self.data_bus
            .set(self.mem[usize::from(self.address_bus.state)]);
    }
}

struct SimState<'a> {
    reset: bool,
    clock: bool,
    cpu: CPU6502<'a>,
    memory: MemorySim<'a>,
}

impl<'a> SimState<'a> {
    fn step(self: &mut Self, clock: bool, reset: bool) {
        self.clock = clock;
        self.reset = reset;
        self.connect_components();
    }

    fn connect_components(self: &mut Self) {
        if self.reset {
            self.cpu.pin_state.reset_en.set(SignalState::LOW)
        } else {
            self.cpu.pin_state.reset_en.set(SignalState::HIGH)
        }

        self.memory.set_address_bus(&self.cpu.pin_state.address_bus);
        self.cpu.pin_state.data_bus.set(self.memory.data_bus.state);

        if self.clock {
            self.cpu.pin_state.clock.set(SignalState::HIGH);
        } else {
            self.cpu.pin_state.clock.set(SignalState::LOW);
        }
        self.cpu.clockTick(self.clock);
    }
}

fn main() {
    run_test();
    
    let mut raw_mem = [0; 0xFFFF + 1];

    for x in 0..0xFFFF {
        let num = x % 4;
        raw_mem[x] = match num {
            0 => 0xDE,
            1 => 0xAD,
            2 => 0xBE,
            3 => 0xEF,
            _ => 0x00,
        };
    }

    let mut sim = SimState {
        reset: false,
        clock: false,
        cpu: CPU6502::new(),
        memory: MemorySim::new(raw_mem),
    };

    // reset sequence
    sim.step(true, true);
    sim.step(false, false);

    let mut tick = true;
    loop {
        sim.step(tick, false);
        tick = !tick;
    }
}
