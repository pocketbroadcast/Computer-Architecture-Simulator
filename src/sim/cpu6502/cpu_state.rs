pub struct CPUState {
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub sp: u16, // 8 bit setable with 0x0100 memory offset
    pub pc: u16,
}

impl CPUState {
    pub fn new() -> Self {
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

    pub fn reset(self: &mut Self) -> &mut Self {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0x0000; // gets loaded from FFFC (low byte) FFFD (high byte) -> little endian,
        self.sp = 0x01FF;

        self
    }
}
