use iced::{
    button, executor, slider, time, Align, Application, Button, Clipboard, Color, Column, Command,
    Container, Element, Length, Row, Slider, Subscription, Text,
};

use super::bitbutton;
use super::circle;
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
    reset: button::State,
    input_state: bitbutton::InputHalfByte,
    output_state: bitbutton::InputHalfByte,
    rom_state: bitbutton::RomTable,
    hoge: bool,
    slider: slider::State,
    period: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Reset,
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
        let td4 = TD4 {
            cpu: Emulator::new("prg.bin"),
            period: 300,
            ..TD4::default()
        };

        (td4, Command::none())
    }

    fn title(&self) -> String {
        String::from("TD4")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Reset => {
                self.cpu.reg = td4_emu::reg::Reg::default();
                self.cpu.port = td4_emu::port::Port::default();
            }
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
                println!("Input = 0b{:04b}", self.cpu.port.input);
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

        let reset = Button::new(&mut self.reset, Text::new("Reset"))
            .width(Length::from(300))
            .padding(10)
            .on_press(Message::Reset)
            .style(self.theme);

        let controls = Row::new().spacing(5).push(run).push(stop).push(step);
        let slider = Slider::new(
            &mut self.slider,
            100.0..=1000.0,
            self.period as f64,
            Message::SliderChanged,
        );

        let input = self.input_state.crate_layout(&self.cpu.port.input);
        let input_info = Row::new()
            .spacing(20)
            .push(Text::new("Input Port").width(Length::from(200)))
            .push(input)
            .push(Text::new(format!("0x{:1X}", &self.cpu.port.input)).width(Length::from(100)))
            .align_items(Align::End);

        let output_info = reg_info(&self.cpu.port.output, "Output Port".to_string());

        let rega_info = reg_info(&self.cpu.reg.a, "Register A".to_string());

        let regb_info = reg_info(&self.cpu.reg.b, "Register B".to_string());

        let pc_info = reg_info(&self.cpu.reg.pc, "Program Counter".to_string());

        let carry = circle::Circle::new(10.0, bit2color(&self.cpu.reg.flag));
        let carry_info = Row::new()
            .spacing(20)
            .push(Text::new("Carry Flag").width(Length::from(200)))
            .push(carry)
            .push(Text::new(format!("{}", &self.cpu.reg.flag)).width(Length::from(100)))
            .align_items(Align::End);

        let io = Column::new()
            .spacing(20)
            .max_width(400)
            .push(pc_info)
            .push(rega_info)
            .push(regb_info)
            .push(carry_info)
            .push(output_info)
            .push(input_info)
            .push(slider)
            .push(controls)
            .push(reset)
            .align_items(Align::Center);

        let rom = self.rom_state.create_layout(&self.cpu.prg);
        let pc = self.cpu.reg.pc;
        let rom_control =
            rom.into_iter()
                .enumerate()
                .fold(Column::new().spacing(5), |col, (i, btn)| {
                    col.push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new(format!("{}:", i)))
                            .push(btn)
                            .push(
                                Container::new(circle::Circle::new(
                                    10.0,
                                    if pc == i as u8 {
                                        Color::from_rgb(1.0, 0.0, 0.0)
                                    } else {
                                        Color::BLACK
                                    },
                                ))
                                .center_x()
                                .center_y(),
                            ),
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

pub fn led4bit(halfbyte: &u8) -> Row<Message> {
    (0..4)
        .into_iter()
        .rev()
        .fold(Row::new().spacing(1), |row, i| {
            row.push(circle::Circle::new(
                10.0,
                bit2color(&((halfbyte & (0x01 << i)) != 0)),
            ))
        })
}

pub fn reg_info(reg: &u8, text: String) -> Row<Message> {
    let led = led4bit(reg);
    let info = Row::new()
        .spacing(20)
        .push(Text::new(&text).width(Length::from(200)))
        .push(led)
        .push(Text::new(format!("0x{:1X}", reg)).width(Length::from(100)))
        .align_items(Align::End);
    info
}

fn bit2color(bit: &bool) -> Color {
    if *bit {
        Color::from_rgb(1.0, 0.0, 0.0)
    } else {
        Color::BLACK
    }
}
