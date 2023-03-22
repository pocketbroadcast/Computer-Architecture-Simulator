use super::generic::Connection;

pub struct Component {
    input0_in: Connection<bool>,
    input1_in: Connection<bool>,
    output_out: Connection<bool>,
}

impl Component {
    pub fn new(
        input0_in: Connection<bool>,
        input1_in: Connection<bool>,
        output_out: Connection<bool>,
    ) -> Self {
        Self {
            input0_in: input0_in,
            input1_in: input1_in,
            output_out: output_out,
        }
    }

    pub fn tick(&mut self) {
        let in0 = self.input0_in.read_copy();
        let in1 = self.input1_in.read_copy();
        
        self.output_out.write_copy(!(in0 & in1));
    }
}
