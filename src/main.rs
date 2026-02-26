use std::io;
mod app;
use crate::app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::default();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
