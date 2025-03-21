use std::io;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
    prelude::Alignment,
};

pub fn draw(terminal: &mut Terminal<impl ratatui::backend::Backend>, data: &[u8], cursor: usize, filename: &str) -> io::Result<()> {
    terminal.draw(|frame| {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(size);

        let lines: Vec<Line> = data.chunks(16).enumerate().map(|(i, chunk)| {
            let addr = Span::raw(format!("{:08x} │ ", i * 16));
            let mut hex: Vec<Span> = Vec::new();
            let mut ascii_repr = String::new();

            for (j, &byte) in chunk.iter().enumerate() {
                let hex_str = format!("{:02x}", byte);
                let index = i * 32 + j * 2;

                for (k, ch) in hex_str.chars().enumerate() {
                    let cur_idx = index + k;
                    let style = if cur_idx == cursor {
                        Style::default().bg(Color::White).fg(Color::Black)
                    } else {
                        Style::default()
                    };
                    hex.push(Span::styled(ch.to_string(), style));
                }

                if j < 15 {
                    let space = if j % 4 == 3 { "   " } else { " " };
                    hex.push(Span::raw(space));
                }

                ascii_repr.push(if byte.is_ascii_graphic() || byte == b' ' { byte as char } else { '.' });
            }

            let missing_bytes = 16 - chunk.len();
            let padding = missing_bytes * 3;
            let spaces = " ".repeat(padding);

            let ascii = Span::raw(format!("{} │ {}", spaces, ascii_repr));

            Line::from(vec![addr]
                .into_iter()
                .chain(hex)
                .chain(std::iter::once(ascii))
                .collect::<Vec<_>>())
        }).collect();

        frame.render_widget(Paragraph::new(lines).block(Block::default().borders(Borders::ALL)), layout[0]);

        let progress = if data.is_empty() { 100 } else { (cursor / 2 * 100) / data.len() };
        let status_text = format!("{} \t 0x{:08x} \t {:3}%", filename, cursor / 2, progress);
        let status_bar = Paragraph::new(Line::from(Span::raw(status_text))).alignment(Alignment::Center);
        frame.render_widget(status_bar, layout[1]);
    })?;
    Ok(())
}
