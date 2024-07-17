mod rpc;
mod message;
mod tui;

use std::{fmt::format, io};

use ratatui::{
    buffer::Buffer,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.render_frame(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, f: &mut Frame) {
        self.render(f.size(), f.buffer_mut())
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Counter".bold());
        let instructions = Title::from(Line::from(vec![
            "Dec".into(),
            "<Left>".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title)
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Counter: ".into(),
                self.counter.to_string().bold().yellow().into(),
            ])
        ]);

        Paragraph::new(counter_text).block(block).render(area, buf);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn inc(&mut self) {
        self.counter += 1;
    }

    fn dec(&mut self) {
        self.counter -= 1;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.dec(),
            KeyCode::Right => self.inc(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;

    // let _ = rpc::run(); 
    app_result
}