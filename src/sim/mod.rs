pub mod cpu6502;
pub mod generic;
pub mod nand;
pub mod sim_ram;

use generic::Signal;
use std::{thread, time};

pub fn run_sim1(memory: [u8; 0xFFFF + 1]) {
    let clk_signal: Signal<bool> = Signal::new();
    let rst_signal: Signal<bool> = Signal::new();
    let rwb_signal: Signal<bool> = Signal::new();
    let databus_signal: Signal<u8> = Signal::new();
    let addressbus_signal: Signal<u16> = Signal::new();

    let mut cpu = cpu6502::Component::new(
        clk_signal.create_connection(),
        rst_signal.create_connection(),
        databus_signal.create_connection(),
        rwb_signal.create_connection(),
        addressbus_signal.create_connection(),
    );

    let rwb_bar_signal: Signal<bool> = Signal::new();
    let ram_cs_bar_signal: Signal<bool> = Signal::new();

    let mut mem = sim_ram::Component::new(
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

    // // reset first cycle (Reset is active low)
    // reset_tx.send(false).unwrap();
    // clk_tx.send(true).unwrap();
    // thread::sleep(clock_sim_duration);

    // //
    // reset_tx.send(true).unwrap();
    // clk_tx.send(false).unwrap();
    // thread::sleep(clock_sim_duration);

    // //

    // // clock cycle
    // loop {
    //     clk_tx.send(true).unwrap();
    //     thread::sleep(clock_sim_duration);
    //     clk_tx.send(false).unwrap();
    //     thread::sleep(clock_sim_duration);
    // }
}

// pub fn run_sim2(){

//     let clock_sim_duration = time::Duration::from_millis(200);
//     let signal : Signal<u8> = Signal::new();

//     let con1 = signal.create_connection();
//     thread::spawn(move ||{
//         for i in 1..=255{
//             // write
//             con1.write_copy(i);
//             println!("Wrote {:?}", i);
//             thread::sleep(clock_sim_duration*2);
//         }
//     });

//     let con2  = signal.create_connection();
//     thread::spawn(move ||{
//         loop{
//             // read
//             let val = con2.read_copy();
//             println!("Read {:?}", val);

//             thread::sleep(clock_sim_duration/2);
//         }
//     });

//     let con3   = signal.create_connection();
//     loop{
//         // read
//         thread::sleep(time::Duration::from_secs(5));

//         // write
//         for i in 1..=255{
//             // write
//             con3.write_copy(i);
//             println!("Wrote 2 {:?}", i);
//             thread::sleep(clock_sim_duration);
//         }
//     }
// }
