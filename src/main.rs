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
        Block, Borders, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
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
}

struct TodoList {
    items: Vec<Task>,
    state: ListState,
}

#[derive(Debug)]
struct Task {
    title: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Upcoming,
    Active,
    Completed,
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') | KeyCode::Char('i') | KeyCode::Char('a') | KeyCode::Char('o') => {
                self.add_task()
            }
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('l')
            | KeyCode::Right
            | KeyCode::Tab
            | KeyCode::Left
            | KeyCode::Char('t') => self.toggle_status(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn add_task(&mut self) {
        todo!()
    }

    fn select_next(&mut self) {
        self.list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.list.state.select_previous();
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

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|todo_item| { ListItem::from(todo_item) })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].status {
                Status::Upcoming => format!("_  {}", self.list.items[i].title),
                Status::Active => format!("☐  {}", self.list.items[i].title),
                Status::Completed => format!("✓  {}", self.list.items[i].title),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw(" Selected Task ").centered())
            .borders(Borders::ALL)
            .border_set(border::LIGHT_TRIPLE_DASHED)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
}

impl Task {
    fn new(status: Status, title: &str) -> Self {
        Self {
            status,
            title: title.to_string(),
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
                (Status::Upcoming, "Write down some tasks"),
                (Status::Active, "Relax"),
                (Status::Completed, "Get list items rendering"),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, title)| Task::new(status, title))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}
