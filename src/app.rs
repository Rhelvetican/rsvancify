use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border::THICK,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};

use crate::tui::Tui;

#[derive(Debug, Default)]
pub struct App {
    counter: i128,
    exit: bool,
}

impl App {
    pub fn run(&mut self, term: &mut Tui) -> Result<()> {
        while !self.exit {
            term.draw(|frame| self.render(frame))?;
            match self.handle_event() {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }
    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
    pub fn handle_event(&mut self) -> Result<()> {
        match read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key(key_event);
            }
            _ => {}
        };
        Ok(())
    }
    pub fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.increment1(),
            KeyCode::Down => self.decrement1(),
            _ => {}
        }
    }
    pub fn increment1(&mut self) {
        self.counter += 1;
    }
    pub fn decrement1(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        } else {
            self.counter = i128::MAX;
        }
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Counter");
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Down>".blue().bold(),
            " Increment ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(THICK);
        let counter = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);
        Paragraph::new(counter)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
