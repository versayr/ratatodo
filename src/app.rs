use std::{io, option::Option};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color,
        palette::tailwind::GREEN,
    },
    text::Line,
    widgets::{
        ListItem, ListState, 
        Widget,
    },
    DefaultTerminal, Frame,
};

const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c300;

pub struct App {
    exit: bool,
    pub list: TodoList,
    mode: Mode,
    pub currently_editing: CurrentlyEditing,
    editing_existing_item: Index,
    pub title_field: String,
    pub info_field: String,
}

pub struct TodoList {
    pub items: Vec<Task>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub info: String,
    pub mode: Status,
}

struct Index {
    index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Upcoming,
    Active,
    Completed,
}

pub enum CurrentlyEditing {
    Title,
    Info,
}

enum Mode {
    View,
    Edit,
    Help,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
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
        match self.mode {
            Mode::View => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('n')
                | KeyCode::Char('i')
                | KeyCode::Char('a')
                | KeyCode::Char('o') => {
                    self.mode = Mode::Edit;
                }
                KeyCode::Char('j') | KeyCode::Down => self.list.state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.list.state.select_previous(),
                KeyCode::Char('h') => self.mode = Mode::Help,
                KeyCode::Char('e') => self.edit_task(),
                KeyCode::Delete | KeyCode::Backspace | KeyCode::Char('d') => self.delete_task(),
                KeyCode::Char('l')
                | KeyCode::Right
                | KeyCode::Tab
                | KeyCode::Left
                | KeyCode::Char('t') => self.toggle_mode(),
                _ => {}
            },
            Mode::Edit => match key_event.code {
                KeyCode::Esc => self.mode = Mode::View,
                KeyCode::Tab | KeyCode::Up | KeyCode::Down => self.toggle_editing_field(),
                KeyCode::Backspace => match self.currently_editing {
                    CurrentlyEditing::Title => {
                        self.title_field.pop();
                    }
                    CurrentlyEditing::Info => {
                        self.info_field.pop();
                    }
                },
                KeyCode::Enter => match self.currently_editing {
                    CurrentlyEditing::Title => self.currently_editing = CurrentlyEditing::Info,
                    CurrentlyEditing::Info => {
                        self.new_task();
                        self.mode = Mode::View;
                    }
                },
                KeyCode::Char(value) => match self.currently_editing {
                    CurrentlyEditing::Title => {
                        self.title_field.push(value);
                    }
                    CurrentlyEditing::Info => {
                        self.info_field.push(value);
                    }
                },
                _ => {}
            },
            Mode::Help => {
                if key_event.code == KeyCode::Esc {
                    self.mode = Mode::View
                }
            }
        }
    }

    fn new_task(&mut self) {
        if !self.title_field.is_empty() {
            if let Some(i) = self.editing_existing_item.index {
                self.list.items[i].title = self.title_field.clone();
                self.list.items[i].info = self.info_field.clone();
            } else {
                self.list.items.push(Task::new(
                    Status::Upcoming,
                    &self.title_field,
                    &self.info_field,
                ));
            }
            self.title_field = "".into();
            self.info_field = "".into();
            self.currently_editing = CurrentlyEditing::Title;
            self.editing_existing_item = Index { index: None };
        }
    }

    fn edit_task(&mut self) {
        if let Some(i) = self.list.state.selected() {
            self.title_field = self.list.items[i].title.clone();
            self.info_field = self.list.items[i].info.clone();
            self.editing_existing_item = Index { index: Some(i) };
            self.mode = Mode::Edit;
        }
    }

    fn delete_task(&mut self) {
        if let Some(i) = self.list.state.selected() {
            self.list.items.remove(i);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn toggle_mode(&mut self) {
        if let Some(i) = self.list.state.selected() {
            self.list.items[i].mode = match self.list.items[i].mode {
                Status::Upcoming => Status::Active,
                Status::Active => Status::Completed,
                Status::Completed => Status::Upcoming,
            }
        }
    }

    fn toggle_editing_field(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Title => self.currently_editing = CurrentlyEditing::Info,
            CurrentlyEditing::Info => self.currently_editing = CurrentlyEditing::Title,
        }
    }
}

impl Widget for &mut App {
     fn render(self, area: Rect, buf: &mut Buffer) {
         match self.mode {
             Mode::View => self.render_view_mode(area, buf),
             Mode::Edit => self.render_edit_mode(area, buf),
             Mode::Help => self.render_help_mode(area, buf),
         }
     }
 }
 
 impl Task {
     fn new(mode: Status, title: &str, info: &str) -> Self {
         Self {
             mode,
             title: title.to_string(),
             info: info.to_string(),
         }
     }
 }
 
 impl From<&Task> for ListItem<'_> {
     fn from(value: &Task) -> Self {
         let line = match value.mode {
             Status::Upcoming => Line::raw(format!(" _ {}", value.title)),
             Status::Active => Line::raw(format!(" ☐ {}", value.title)),
             Status::Completed => {
                 Line::styled(format!(" ✓ {}", value.title), COMPLETED_TEXT_FG_COLOR)
             }
         };
         ListItem::new(line)
     }
 }
 
 impl Default for App {
     fn default() -> Self {
         Self {
             exit: false,
             list: TodoList::from_iter([]),
             mode: Mode::View,
             title_field: "".into(),
             info_field: "".into(),
             currently_editing: CurrentlyEditing::Title,
             editing_existing_item: Index { index: None },
         }
     }
 }
 
 impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
     fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
         let items = iter
             .into_iter()
             .map(|(mode, title, info)| Task::new(mode, title, info))
             .collect();
         let state = ListState::default();
         Self { items, state }
     }
 }
