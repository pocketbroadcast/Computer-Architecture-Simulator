mod sim;

use crate::sim::run_sim1;

fn main() {
    let mut raw_mem = [0; 0xFFFF + 1];

    raw_mem[0x8000] = 0xA9; // LDA #0xAB
    raw_mem[0x8001] = 0xAB;
    raw_mem[0x8002] = 0x29; // AND #0xA0
    raw_mem[0x8003] = 0xA0;
    raw_mem[0x8004] = 0x0A; // ASL A

    raw_mem[0x8005] = 0x8D; // STA $0001
    raw_mem[0x8006] = 0x00;
    raw_mem[0x8007] = 0x01;

    raw_mem[0x8008] = 0x20; // JSR $A0B0
    raw_mem[0x8009] = 0xB0;
    raw_mem[0x800A] = 0xA0;

    raw_mem[0x800B] = 0x4C; // JMP $8000
    raw_mem[0x800C] = 0x00;
    raw_mem[0x800D] = 0x80;
    
    raw_mem[0xA0B0] = 0x8D; // STA $0002
    raw_mem[0xA0B1] = 0x00;
    raw_mem[0xA0B2] = 0x02;

    raw_mem[0xA0B3] = 0x60; // RTS

    raw_mem[0xfffc] = 0x00; // .org 8000
    raw_mem[0xfffd] = 0x80;

    run_sim1(raw_mem);
}
