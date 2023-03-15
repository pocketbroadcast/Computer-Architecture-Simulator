mod sim;

use crate::sim::run_sim1;

fn main() {
    
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

    run_sim1(raw_mem);
}
