use std::{io, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::tailwind::{GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols::border,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c300;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::default();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
    list: TodoList,
    mode: Mode, 
    currently_editing: CurrentlyEditing,
    title_field: String, 
    info_field: String,
}

struct TodoList {
    items: Vec<Task>,
    state: ListState,
}

#[derive(Debug)]
struct Task {
    title: String,
    info: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Upcoming,
    Active,
    Completed,
}

enum CurrentlyEditing {
    Title,
    Info
}

enum Mode {
    View,
    Edit,
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
            Mode::View => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('n') | KeyCode::Char('i') | KeyCode::Char('a') | KeyCode::Char('o') => {
                        self.mode = Mode::Edit;
                    }
                    KeyCode::Char('j') | KeyCode::Down => self.list.state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.list.state.select_previous(),
                    KeyCode::Char('l')
                        | KeyCode::Right
                        | KeyCode::Tab
                        | KeyCode::Left
                        | KeyCode::Char('t') => self.toggle_status(),
                    _ => {}
                }
            },
            Mode::Edit => {
                match key_event.code {
                    KeyCode::Esc => {
                        // TODO handle discarding changes
                        self.mode = Mode::View;
                    },
                    KeyCode::Tab | KeyCode::Up | KeyCode::Down => todo!(),
                    _ => {}
                }
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.list.state.selected() {
            self.list.items[i].status = match self.list.items[i].status {
                Status::Upcoming => Status::Active,
                Status::Active => Status::Completed,
                Status::Completed => Status::Upcoming,
            }
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" ").centered())
            .borders(Borders::TOP)
            .border_set(border::EMPTY);

        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|todo_item| ListItem::from(todo_item))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line<'_>> = vec![];
        // We get the info depending on the item's state.
        let task = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].status {
                Status::Upcoming => format!("{} ", self.list.items[i].title),
                Status::Active => format!("{} ", self.list.items[i].title),
                Status::Completed => format!("{} ", self.list.items[i].title),
            }
        } else {
            " Nothing selected... ".to_string()
        };

        let info = if let Some(i) = self.list.state.selected() {
            &self.list.items[i].info
        } else {
            ""
        };

        let task_status = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].status {
                Status::Upcoming => "> Status - Upcoming ",
                Status::Active => "> Status - Active ",
                Status::Completed => "> Status - Completed ",
            }
        } else {
            ""
        };

        lines.push(Line::from(task));
        lines.push(Line::from(info));

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::from(task_status).bold())
            .borders(Borders::TOP)
            .border_set(border::LIGHT_TRIPLE_DASHED)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(lines)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatodo ".bold());
        let instructions = Line::from(vec![
            " [".into(),
            "N".blue().bold(),
            "]ew Task".into(),
            " [".into(),
            "M".blue().bold(),
            "]ark Completed".into(),
            " [".into(),
            "Q".blue().bold(),
            "]uit ".into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(3)])
            .split(Block::inner(&block, area));

        block.render(area, buf);
        self.render_list(layout[0], buf);
        self.render_selected_item(layout[1], buf);
    }

    fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatodo ".bold());
        let instructions = Line::from(vec![
            " [".into(),
            "Esc".blue().bold(),
            "] Discard Changes".into(),
            " [".into(),
            "Tab".blue().bold(),
            "] Switch Field ".into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_type(BorderType::Rounded);

        let title_block = Block::bordered()
            .title(Line::raw(" Task Title "))
            .border_type(BorderType::Rounded);

        let info_block = Block::bordered()
            .title(Line::raw(" Task Details "))
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(Block::inner(&block, area));

        // TODO highlight currently selected block 
        // TODO render typed task title/info in blocks 
        // TODO set up editing existing tasks

        block.render(area, buf);
        title_block.render(layout[0], buf);
        info_block.render(layout[1], buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            Mode::View => self.render_view_mode(area, buf),
            Mode::Edit => self.render_edit_mode(area, buf),
        }
    }
}

impl Task {
    fn new(status: Status, title: &str, info: &str) -> Self {
        Self {
            status,
            title: title.to_string(),
            info: info.to_string(),
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Upcoming => Line::styled(format!(" _ {}", value.title), TEXT_FG_COLOR),
            Status::Active => Line::styled(format!(" ☐ {}", value.title), TEXT_FG_COLOR),
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
            list: TodoList::from_iter([
                (
                    Status::Upcoming,
                    "Write down some tasks",
                    "Use this todo app",
                ),
                (Status::Active, "Relax", "You've been doing good work"),
                (
                    Status::Completed,
                    "Get list items rendering",
                    "Task lists need items rendering",
                ),
            ]),
            mode: Mode::View,
            title_field: "".into(),
            info_field: "".into(),
            currently_editing: CurrentlyEditing::Title,
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, title, info)| Task::new(status, title, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}
