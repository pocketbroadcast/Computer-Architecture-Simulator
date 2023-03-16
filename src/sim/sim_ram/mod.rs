use super::generic::Connection;

// inspired by HM62256B Series
pub struct Component {
    address_bus_in: Connection<u16>,
    data_bus_inout: Connection<u8>,
    write_en_bar_in: Connection<bool>,
    output_en_bar_in: Connection<bool>,
    chip_sel_bar_in: Connection<bool>,

    memory: [u8; 0xFFFF + 1],
}

impl Component {
    pub fn new(
        address_bus_in: Connection<u16>,
        data_bus_inout: Connection<u8>,
        write_en_bar_in: Connection<bool>,
        output_en_bar_in: Connection<bool>,
        chip_sel_bar_in: Connection<bool>,
        memory: [u8; 0xFFFF + 1],
    ) -> Self {
        Self {
            address_bus_in: address_bus_in,
            data_bus_inout: data_bus_inout,
            write_en_bar_in: write_en_bar_in,
            output_en_bar_in: output_en_bar_in,
            chip_sel_bar_in: chip_sel_bar_in,
            memory: memory,
        }
    }

    pub fn tick(&mut self) {
        let chip_select_bar = self.chip_sel_bar_in.read_copy();

        if chip_select_bar {
            return;
        }

        let address = self.address_bus_in.read_copy();

        let output_en_bar = self.output_en_bar_in.read_copy();
        if !output_en_bar {
            self.data_bus_inout
                .write_copy(self.memory[usize::from(address)]);

            return;
        }

        let write_en_bar = self.write_en_bar_in.read_copy();

        if !write_en_bar {
            self.memory[usize::from(address)] = self.data_bus_inout.read_copy();

            return;
        }
    }
}
