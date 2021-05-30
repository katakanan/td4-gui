use iced::{
    button, executor, slider, time, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, Length, Row, Slider, Subscription, Text,
};

use super::bitbutton;
use super::style;
use td4_emu::emulator::Emulator;

#[derive(Debug, Eq, PartialEq)]
enum State {
    Idle,
    Active,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

#[derive(Debug, Default)]
pub struct TD4 {
    theme: style::Theme,
    cpu: Emulator,
    state: State,
    run: button::State,
    stop: button::State,
    step: button::State,
    input_state: bitbutton::InputHalfByte,
    output_state: bitbutton::InputHalfByte,
    rom_state: bitbutton::RomTable,
    hoge: bool,
    slider: slider::State,
    period: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Run,
    Step,
    Stop,
    RomEdit(usize, u8, bool),
    InputEdit(u8, bool),
    SliderChanged(f64),
}

impl TD4 {
    pub fn step(&mut self) {
        let (opecode, operand) = self.cpu.fetch_decode();
        // println!("{:?} {:?}", opecode, operand);
        let next_pc = self.cpu.exec_mut(&opecode, operand);
        self.cpu.reg.pc = next_pc;
        // println!("0b_{:04b}", self.cpu.port.output);
    }

    pub fn show(&self) {
        println!("0b{:04b}", self.cpu.port.output);
    }
}

impl Application for TD4 {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let cpu = Emulator::new("prg.bin");
        let mut td4 = TD4::default();
        td4.cpu = cpu;
        td4.period = 1000;
        (td4, Command::none())
    }

    fn title(&self) -> String {
        String::from("TD4")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                self.step();
                self.show();
            }
            Message::Run => {
                self.state = State::Active;
            }

            Message::Step => {
                if self.state == State::Idle {
                    self.step();
                    self.show();
                }
            }
            Message::InputEdit(bit, now) => {
                self.cpu.port.input =
                    (self.cpu.port.input & !(0x01 << bit)) | ((!now as u8) << bit);
                println!("0b{:04b}", self.cpu.port.input);
            }
            Message::RomEdit(addr, bit, now) => {
                let newbyte = (self.cpu.prg.mem[addr] & !(0x01 << bit)) | ((!now as u8) << bit);
                println!(
                    "Rom[{:2}] = 0b{hi:04b}_{lo:04b}",
                    addr,
                    hi = ((newbyte & 0xF0) >> 4),
                    lo = (newbyte & 0x0F)
                );

                self.cpu.prg.mem[addr] = newbyte;
            }
            Message::Stop => {
                self.state = State::Idle;
            }
            Message::SliderChanged(value) => {
                self.period = value as u64;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Active => {
                time::every(std::time::Duration::from_millis(self.period)).map(|_| Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let run = Button::new(&mut self.run, Text::new("Run"))
            .padding(10)
            .on_press(Message::Run)
            .style(self.theme);

        let step = Button::new(&mut self.step, Text::new("Step"))
            .padding(10)
            .on_press(Message::Step)
            .style(self.theme);

        let stop = Button::new(&mut self.stop, Text::new("Stop"))
            .padding(10)
            .on_press(Message::Stop)
            .style(self.theme);

        let controls = Row::new().spacing(5).push(run).push(stop).push(step);
        let slider = Slider::new(
            &mut self.slider,
            100.0..=1000.0,
            self.period as f64,
            Message::SliderChanged,
        );

        let input = self.input_state.crate_layout(&self.cpu.port.input);
        // println!("{}", self.cpu.prg.mem.len()); //16

        let io = Column::new()
            .spacing(20)
            .max_width(400)
            .push(input)
            .push(slider)
            .push(controls)
            .align_items(Align::Center);

        let rom = self.rom_state.create_layout(&self.cpu.prg);
        let rom_control =
            rom.into_iter()
                .enumerate()
                .fold(Column::new().spacing(5), |col, (i, btn)| {
                    col.push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new(format!("{}:", i)))
                            .push(btn),
                    )
                    .align_items(Align::End)
                });

        let content = Row::new()
            .spacing(20)
            .push(io)
            .push(rom_control)
            .align_items(Align::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
