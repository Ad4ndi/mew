use std::{io, time::Duration};
use crossterm::{event::{self, KeyCode, KeyEvent}};

pub fn handle_input(cursor: &mut usize, data: &mut [u8]) -> io::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => return Ok(false),
                KeyCode::Left if *cursor > 0 => *cursor -= 1,
                KeyCode::Right if *cursor < data.len() * 2 - 1 => *cursor += 1,
                KeyCode::Up if *cursor >= 32 => *cursor -= 32,
                KeyCode::Down if *cursor + 32 < data.len() * 2 => *cursor += 32,
                KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                    let byte_index = *cursor / 2;
                    let shift = if *cursor % 2 == 0 { 4 } else { 0 };
                    data[byte_index] = (data[byte_index] & (0x0F << (4 - shift))) | (c.to_digit(16).unwrap() as u8) << shift;
                }
                _ => {}
            }
        }
    }
    Ok(true)
}
