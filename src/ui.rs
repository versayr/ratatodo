use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{Style, Stylize, Modifier},
    symbols::border,
    text::Line,
    widgets::{
        Block,
        Borders,
        BorderType,
        HighlightSpacing,
        List,
        ListItem,
        Padding,
        Paragraph,
        StatefulWidget,
        Widget,
        Wrap,
    },
};
use crate::app::{
    App,
    CurrentlyEditing,
    Status,
};

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
// const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c300;


impl App {
    pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatodo ".bold());
        let instructions = Line::from(vec![
            " [".into(),
            "N".blue().bold(),
            "]ew Task".into(),
            " [".into(),
            "E".blue().bold(),
            "]dit".into(),
            " [".into(),
            "H".blue().bold(),
            "]elp".into(),
            " [".into(),
            "Q".blue().bold(),
            "]uit ".into(),
        ]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .padding(Padding::vertical(1))
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(3)])
            .split(Block::inner(&block, area));

        block.render(area, buf);
        self.render_list(layout[0], buf);
        self.render_selected_item(layout[1], buf);
    }

    pub fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatodo ".bold());
        let instructions = Line::from(vec![
            " [".into(),
            "Esc".blue().bold(),
            "] Discard Changes".into(),
            " [".into(),
            "Tab".blue().bold(),
            "] Switch Field".into(),
            " [".into(),
            "Enter".blue().bold(),
            "] Submit".into(),
        ]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(Block::inner(&block, area));

        let title_border_style = match self.currently_editing {
            CurrentlyEditing::Title => BorderType::Double,
            CurrentlyEditing::Info => BorderType::Plain,
        };

        let info_border_style = match self.currently_editing {
            CurrentlyEditing::Info => BorderType::Double,
            CurrentlyEditing::Title => BorderType::Plain,
        };

        let title_block = Block::bordered()
            .title(Line::raw(" Task Title "))
            .border_type(title_border_style)
            .padding(Padding::uniform(1));

        let info_block = Block::bordered()
            .title(Line::raw(" Task Details "))
            .border_type(info_border_style)
            .padding(Padding::uniform(1));

        let title_field = Paragraph::new(self.title_field.clone())
            .wrap(Wrap { trim: true })
            .block(title_block);

        let info_field = Paragraph::new(self.info_field.clone())
            .wrap(Wrap { trim: true })
            .block(info_block);

        block.render(area, buf);
        title_field.render(layout[0], buf);
        info_field.render(layout[1], buf);
    }

    pub fn render_help_mode(&mut self, area: Rect, buf: &mut Buffer) {
        Line::raw("Help Screen").render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|todo_item| ListItem::from(todo_item))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line<'_>> = vec![];
        // We get the info depending on the item's state.
        let task = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].mode {
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

        let task_mode = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].mode {
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
            .title(Line::from(task_mode).bold())
            .borders(Borders::TOP)
            .border_set(border::LIGHT_TRIPLE_DASHED)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .render(area, buf);
        }
}


