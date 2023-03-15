pub struct PinState {
    pub clock: bool,
    pub reset_en: bool,

    pub data_bus: u8,
    pub rwb: bool,
    pub address_bus: u16,
}

impl PinState {
    pub fn new() -> Self {
        let mut new_state = Self {
            clock: false,
            reset_en: false,
            data_bus: 0x00,
            rwb: true,
            address_bus: 0xFFFF,
        };

        new_state.reset();

        new_state
    }

    pub fn reset(self: &mut Self) -> &mut Self {
        self.clock = false;
        self.reset_en = false;
        self.data_bus = 0x00;
        self.rwb = true;
        self.address_bus = 0xFFFF;

        self
    }
}
