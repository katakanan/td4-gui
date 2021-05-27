extern crate iced;
extern crate td4_emu;

mod td4_gui;

use crate::iced::Application;

fn main() -> iced::Result {
    println!("Hello, world!");
    // let mut emu = td4::Emulator::new("prg.bin");

    // println!("{:?}", emu);

    // for _ in 0..10 {
    //     let (opecode, operand) = emu.fetch_decode();
    //     println!("{:?} {:?}", opecode, operand);
    //     let next_pc = emu.exec_mut(&opecode, operand);
    //     emu.reg.pc = next_pc;
    //     println!("{:?}", emu);
    // }
    td4_gui::TD4::run(iced::Settings::default())
}
