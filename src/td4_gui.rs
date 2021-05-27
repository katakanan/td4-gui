use iced::{
    button, executor, time, Align, Application, Button, Clipboard, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Subscription, Text,
};

use td4_emu::emulator::Emulator;

#[derive(Debug)]
pub struct TD4 {
    cpu: Emulator,
    run: button::State,
    stop: button::State,
    step: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Run,
    Step,
    Stop,
}

impl Application for TD4 {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let cpu = Emulator::new("prg.bin");
        let td4 = TD4 {
            cpu,
            run: button::State::new(),
            stop: button::State::new(),
            step: button::State::new(),
        };

        (td4, Command::none())
    }

    fn title(&self) -> String {
        String::from("TD4")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                let (opecode, operand) = self.cpu.fetch_decode();
                println!("{:?} {:?}", opecode, operand);
                let next_pc = self.cpu.exec_mut(&opecode, operand);
                self.cpu.reg.pc = next_pc;
                println!("{}", self.cpu.port.output);
                // println!("{:?}", emu
            }
            Message::Run => {
                println!("Run");
            }
            Message::Step => {
                println!("Step");
            }
            Message::Stop => {
                println!("Stop");
            }
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
            )
            .min_width(80)
            .padding(10)
            .style(style)
        };

        let run_button =
            button(&mut self.run, "Run", style::Button::Primary).on_press(Self::Message::Run);
        let stop_button =
            button(&mut self.stop, "Stop", style::Button::Primary).on_press(Self::Message::Stop);
        let step_button =
            button(&mut self.step, "Step", style::Button::Primary).on_press(Self::Message::Step);

        let controls = Row::new()
            .spacing(20)
            .push(run_button)
            .push(stop_button)
            .push(step_button);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
