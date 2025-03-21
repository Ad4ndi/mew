use std::{fs, io};
use std::io::stdout;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crate::{input, render};

pub struct HexEditor {
    filename: String,
    data: Vec<u8>,
    cursor: usize,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl HexEditor {
    pub fn new(filename: String, data: Vec<u8>) -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { filename, data, cursor: 0, terminal })
    }

    pub fn run(&mut self) -> io::Result<()> {
        while input::handle_input(&mut self.cursor, &mut self.data[..])? {
            render::draw(&mut self.terminal, &self.data, self.cursor, &self.filename)?;
        }
        self.cleanup()
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        fs::write(&self.filename, &self.data)?;
        Ok(())
    }
}
