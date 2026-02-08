use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App { exit: false };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') => todo!(),
            KeyCode::Char('m') => todo!(),
            KeyCode::Char('h') | KeyCode::Left => todo!(),
            KeyCode::Char('j') | KeyCode::Down => todo!(),
            KeyCode::Char('k') | KeyCode::Up => todo!(),
            KeyCode::Char('l') | KeyCode::Right => todo!(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatodo ".bold());
        let instructions = Line::from(vec![
            " [N]ew Task ".into(),
            "<N>".blue().bold(),
            " [M]ark Completed ".into(),
            "<M>".blue().bold(),
            " [Q]uit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(Block::inner(&block, area));

        Paragraph::new("left")
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[0], buf);

        Paragraph::new("center")
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[1], buf);

        Paragraph::new("right")
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[2], buf);

        block.render(area, buf);
    }
}
