use iced::{button, Button, Row, Text};

use super::style;
use super::td4_gui::Message;

#[derive(Debug, Default, Clone)]
pub struct InputHalfByte {
    pub bit_state: [button::State; 4],
}

impl InputHalfByte {
    pub fn crate_layout(&mut self, value: &u8) -> Row<Message> {
        self.buttons(value)
            .into_iter()
            .fold(Row::new().spacing(1), |row, button| row.push(button))
    }

    fn buttons(&mut self, value: &u8) -> Vec<Button<Message>> {
        self.bit_state
            .iter_mut()
            .enumerate()
            .map(|(i, state)| {
                let bit = value & (0x01 << i) != 0;
                let btn = Button::new(state, bit2text(bit))
                    .on_press(Message::InputEdit(i as u8, bit))
                    .style(bit2style(bit));
                btn
            })
            .rev()
            .collect::<_>()
    }
}

#[derive(Debug, Default, Clone)]
pub struct RomByte {
    pub bit_state: [button::State; 8],
}

impl RomByte {
    pub fn create_layout(&mut self, addr: usize, value: &u8) -> Row<Message> {
        self.buttons(addr, value)
            .into_iter()
            .fold(Row::new().spacing(1), |row, button| row.push(button))
    }

    fn buttons(&mut self, addr: usize, value: &u8) -> Vec<Button<Message>> {
        self.bit_state
            .iter_mut()
            .enumerate()
            .map(|(i, state)| {
                let bit = value & (0x01 << i) != 0;
                let btn = Button::new(state, bit2text(bit))
                    .on_press(Message::RomEdit(addr, i as u8, bit))
                    .style(bit2style(bit));
                btn
            })
            .rev()
            .collect::<_>()
    }
}

#[derive(Debug, Default)]
pub struct RomTable {
    pub table: Vec<RomByte>,
}

impl RomTable {
    pub fn create_layout(&mut self, prg: &td4_emu::mem::Mem) -> Vec<Row<Message>> {
        self.table = vec![RomByte::default(); prg.mem.len()];

        let buttons = self
            .table
            .iter_mut()
            .enumerate()
            .map(|(i, rombyte)| rombyte.create_layout(i, &prg.mem[i]))
            .collect::<Vec<_>>();
        buttons
    }
}

fn bit2text(bit: bool) -> Text {
    if bit {
        Text::new("1")
    } else {
        Text::new("0")
    }
}

fn bit2style(bit: bool) -> style::Theme {
    if bit {
        style::Theme::Light
    } else {
        style::Theme::Dark
    }
}
