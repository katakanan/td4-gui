use iced::{
    button, executor, time, Align, Application, Button, Checkbox, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Subscription, Text,
};

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
    input_state: [button::State; 4],
    hoge: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Run,
    Step,
    Stop,
    RomEdit(usize, u8, bool),
    InputEdit(u8, bool),
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
                println!("{}", bit);
                self.cpu.port.input =
                    (self.cpu.port.input & !(0x01 << bit)) | ((!now as u8) << bit);
                println!("0b{:04b}", self.cpu.port.input);
            }
            Message::RomEdit(addr, bit, now) => {
                self.cpu.port.input =
                    (self.cpu.port.input & !(0x01 << bit)) | ((!now as u8) << bit);
                println!("Rom[{}] =  0b{:04b}", addr, self.cpu.prg.mem[addr]);
            }
            Message::Stop => {
                self.state = State::Idle;
            }
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Active => {
                time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
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

        let inputreg = self.cpu.port.input;
        let inputbtns: Vec<Button<Message>> = self
            .input_state
            .iter_mut()
            .enumerate()
            .map(|(i, state)| {
                let bit = inputreg & (0x01 << i) != 0;
                let btn =
                    Button::new(state, bit2text(bit)).on_press(Message::InputEdit(i as u8, bit));
                btn
            })
            .rev()
            .collect::<_>();

        let input = inputbtns
            .into_iter()
            .fold(Row::new().spacing(1), |row, btn| row.push(btn));

        let controls = Row::new().spacing(5).push(run).push(stop).push(step);

        let content = Column::new()
            .spacing(20)
            .push(input)
            .push(controls)
            .align_items(Align::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn bit2text(bit: bool) -> Text {
    if bit {
        Text::new("1")
    } else {
        Text::new("0")
    }
}
