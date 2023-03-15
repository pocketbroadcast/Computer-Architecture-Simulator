pub mod cpu6502;
pub mod generic;
pub mod sim_ram;

use std::{sync::mpsc, thread, time};

pub fn run_sim1(memory: [u8; 0xFFFF + 1]) {
    let (clk_tx, clk_rx) = mpsc::channel();
    let (reset_tx, reset_rx) = mpsc::channel();
    let (databus_tx, databus_rx) = mpsc::channel();
    let (addressbus_tx, addressbus_rx) = mpsc::channel();

    let comp = cpu6502::Component::new(clk_rx, reset_rx, databus_rx, addressbus_tx);
    comp.start();

    let mem = sim_ram::Component::new(addressbus_rx, databus_tx, memory);
    mem.start();

    thread::sleep(time::Duration::from_millis(1000));

    let clock_cycle = time::Duration::from_millis(10);
    let clock_sim_duration = clock_cycle / 2;
    
    println!("Setup completed - Simulation starts!");

    // reset first cycle (Reset is active low)
    reset_tx.send(false).unwrap();
    clk_tx.send(true).unwrap();
    thread::sleep(clock_sim_duration);

    //
    reset_tx.send(true).unwrap();
    clk_tx.send(false).unwrap();
    thread::sleep(clock_sim_duration);

    //

    // clock cycle
    loop {
        clk_tx.send(true).unwrap();
        thread::sleep(clock_sim_duration);
        clk_tx.send(false).unwrap();
        thread::sleep(clock_sim_duration);
    }
}
