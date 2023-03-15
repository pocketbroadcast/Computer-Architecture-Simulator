pub mod cpu6502;
pub mod generic;
pub mod sim_ram;

use std::{sync::mpsc, thread, time};

pub fn run_sim1(memory : [u8;0xFFFF+1]) {
    let (clk_tx, clk_rx) = mpsc::channel();
    let (reset_tx, reset_rx) = mpsc::channel();
    let (databus_tx, databus_rx) = mpsc::channel();
    let (addressbus_tx, addressbus_rx) = mpsc::channel();

    let comp = cpu6502::Component::new(clk_rx, reset_rx, databus_rx, addressbus_tx);
    comp.start();

    let mem = sim_ram::Component::new(addressbus_rx, databus_tx, memory);
    mem.start();

    // reset first cycle (Reset is active low)
    reset_tx.send(false).unwrap();
    clk_tx.send(true).unwrap();
    thread::sleep(time::Duration::from_millis(500));

    //
    reset_tx.send(true).unwrap();
    clk_tx.send(false).unwrap();
    thread::sleep(time::Duration::from_millis(500));

    //

    // clock cycle
    loop {
        clk_tx.send(true).unwrap();
        thread::sleep(time::Duration::from_millis(500));
        clk_tx.send(false).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}
