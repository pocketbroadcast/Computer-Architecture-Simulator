use std::{cell::RefCell, rc::Rc};

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

    data_bus: Signal<'a, u8>,
    address_bus: Signal<'a, u16>,
}

impl<'a> PinState<'a> {
    fn new() -> Self {
        Self {
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

    fn inc_pc(self: &mut Self){
        
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

fn main() {
    let cpu = CPU6502::new();
    let x = Rc::new(RefCell::new(cpu));

    let mut sig_clock = Signal::new();
    sig_clock.register_on_change(Box::new(|s| {
        if let SignalState::HIGH = s {
            x.borrow_mut().clockTick(true);
        }
    }));

    let mut sig_reset = Signal::new();
    sig_reset.register_on_change(Box::new(|s| {
        x.borrow_mut().pin_state.reset_en.set(s);
    }));

    x.borrow_mut()
        .pin_state
        .address_bus
        .register_on_change(Box::new(|s| {
            println!("address bus changed to: {:?}", s);
        }));

    sig_reset.set(SignalState::LOW);
    sig_clock.set(SignalState::HIGH);
    sig_reset.set(SignalState::HIGH);
    sig_clock.set(SignalState::LOW);

    for _ in 1..100 {
        sig_clock.set(SignalState::HIGH);
        sig_clock.set(SignalState::LOW);
    }

    sig_reset.set(SignalState::LOW);
    sig_clock.set(SignalState::HIGH);
    sig_reset.set(SignalState::HIGH);
    sig_clock.set(SignalState::LOW);
    
    loop {
        sig_clock.set(SignalState::HIGH);
        sig_clock.set(SignalState::LOW);
    }
    //x.borrow_mut().pin_state.address_bus.set(23);
}
