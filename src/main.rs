mod sim;

use crate::sim::run_sim1;

fn main() {
    
    let mut raw_mem = [0; 0xFFFF + 1];


    raw_mem[0x8000] = 0xA9;   // LDA #0xAB
    raw_mem[0x8001] = 0xAB;
    raw_mem[0x8002] = 0x29;   // AND #0xA0
    raw_mem[0x8003] = 0xA0;
    raw_mem[0x8004] = 0x0A;   // ASL A

    raw_mem[0xfffc] = 0x00;
    raw_mem[0xfffd] = 0x80;
    
    run_sim1(raw_mem);
}
