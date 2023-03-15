use super::generic::Connection;

pub struct Component {
    address_bus_in: Connection<u16>,
    data_bus_inout: Connection<u8>,
    write_en_in: Connection<bool>,

    memory: [u8; 0xFFFF + 1],
}

impl Component {
    pub fn new(
        address_bus_in: Connection<u16>,
        data_bus_inout: Connection<u8>,
        write_en_in: Connection<bool>,
        memory: [u8; 0xFFFF + 1],
    ) -> Self {
        Self {
            address_bus_in: address_bus_in,
            data_bus_inout: data_bus_inout,
            write_en_in: write_en_in,
            memory: memory,
        }
    }

    pub fn tick(&mut self, cs: bool) {
        if !cs {
            return;
        }

        let address = self.address_bus_in.read_copy();
        let write_en = self.write_en_in.read_copy();

        if write_en {
            self.memory[usize::from(address)] = self.data_bus_inout.read_copy();
        } else {
            self.data_bus_inout
                .write_copy(self.memory[usize::from(address)]);
        }
    }
}
