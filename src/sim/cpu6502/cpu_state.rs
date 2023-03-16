pub struct CPUState {
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub sp: u8, // 8 bit setable with 0x0100 memory offset
    pub pc: u16,
    
    pub internal: u16,
}

impl CPUState {
    pub const SP_MEM_OFFSET: u16 = 0x100;
    
    pub fn new() -> Self {
        let mut new_state = Self {
            a: 0,
            x: 0,
            y: 0,

            pc: 0x0000, // gets loaded from FFFC (low byte) FFFD (high byte) -> little endian,
            sp: 0xFF,
            
            internal: 0x0000,
        };

        new_state.reset();

        new_state
    }

    pub fn reset(self: &mut Self) -> &mut Self {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0x0000; // gets loaded from FFFC (low byte) FFFD (high byte) -> little endian,
        self.sp = 0xFF;
        
        self.internal = 0x0000;

        self
    }
}
