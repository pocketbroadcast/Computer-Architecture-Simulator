mod components;

use components::{generic::*,*};
use std::{thread, time};

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

pub fn run_sim1(memory: [u8; 0xFFFF + 1]) {
    let clk_signal: Signal<bool> = Signal::new();
    let rst_signal: Signal<bool> = Signal::new();
    let rwb_signal: Signal<bool> = Signal::new();
    let databus_signal: Signal<u8> = Signal::new();
    let addressbus_signal: Signal<u16> = Signal::new();

    let mut cpu = cpu_6502::Component::new(
        clk_signal.create_connection(),
        rst_signal.create_connection(),
        databus_signal.create_connection(),
        rwb_signal.create_connection(),
        addressbus_signal.create_connection(),
    );

    let rwb_bar_signal: Signal<bool> = Signal::new();
    let ram_cs_bar_signal: Signal<bool> = Signal::new();

    let mut mem = ram_hm62256b::Component::new(
        addressbus_signal.create_connection(),
        databus_signal.create_connection(),
        rwb_signal.create_connection(),
        rwb_bar_signal.create_connection(),
        ram_cs_bar_signal.create_connection(),
        memory,
    );

    let mut nand = nand::Component::new(
        rwb_signal.create_connection(),
        rwb_signal.create_connection(),
        rwb_bar_signal.create_connection(),
    );

    let sim_step_duration = time::Duration::from_millis(10);
    let clock_cycle_sim_steps = 4; // min recommended (2 sim steps per half cycle)

    let clk_sim_connection = clk_signal.create_connection();
    let rst_sim_connection = rst_signal.create_connection();

    clk_sim_connection.write_copy(false);
    rst_sim_connection.write_copy(true);

    println!("Setup completed - Simulation starts!");

    let mut sim_step = 0u32;
    let mut cycle = 0u32;

    loop {
        if sim_step < u32::MAX {
            sim_step += 1;
        } else {
            sim_step = u32::MIN;
        }

        if sim_step % clock_cycle_sim_steps == 0 {
            cycle += 1;
            clk_sim_connection.write_copy(true);
        } else if sim_step % clock_cycle_sim_steps == 2 {
            clk_sim_connection.write_copy(false);
        }

        rst_sim_connection.write_copy(cycle != 1);

        cpu.tick();
        nand.tick();
        mem.tick();

        thread::sleep(sim_step_duration);
    }
}