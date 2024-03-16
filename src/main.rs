use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;
mod tui;

fn main() -> io::Result<()> {
    let mut tui = tui::init()?;
    let app = App::default().run(&mut tui);
    tui::restore()?;
    app
}

#[derive(Debug, Default)]
struct App {
    counter: u128,
    exit: bool,
}

impl App {
    fn run(&mut self, term: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.render(frame))?;
            match self.handle_event() {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }
    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key(key_event);
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.increment1(),
            KeyCode::Down => self.decrement1(),
            _ => {}
        }
    }
    fn increment1(&mut self) {
        self.counter += 1;
    }
    fn decrement1(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        } else {
            self.counter = u128::MAX;
        }
    }
    fn exit(&mut self) {
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
            .border_set(border::THICK);
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
