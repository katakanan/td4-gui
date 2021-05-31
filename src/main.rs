extern crate iced;
extern crate td4_emu;

mod bitbutton;
mod circle;
mod style;
mod td4_gui;

use crate::iced::{Application, Settings};

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

    let window = iced::window::Settings {
        size: (700, 600),
        resizable: false,
        ..iced::window::Settings::default()
    };

    let setting = iced::settings::Settings {
        window,
        ..Settings::default()
    };

    td4_gui::TD4::run(setting)
}
